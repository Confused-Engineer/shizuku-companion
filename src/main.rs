#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#[macro_use]
extern crate windows_service;

use std::ffi::OsString;
use windows_service::{
    service::
    {
        ServiceControl, 
        ServiceControlAccept, 
        ServiceExitCode, 
        ServiceState, 
        ServiceStatus, 
        ServiceType
    }, 

    service_control_handler::
    {
        self, 
        ServiceControlHandlerResult
    }, 

    service_dispatcher};


define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    // The entry point where execution will start on a background thread after a call to
    // `service_dispatcher::start` from `main`.

    // uncomment to use this to add error handling instead of ignoring result
    /* 
    if let Err(_e) = run_service(arguments) {
        Handle errors in some way.
    }
    */
    #[allow(unused_variables)]
    let _ = run_service(arguments);


}

#[allow(unused_variables)]
fn run_service(arguments: Vec<OsString>) -> windows_service::Result<()> {
    //making a sender & receiver to trigger shutdown events later in the code
    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();

    //Manage Stop/Shutdown events to handle.
    //Windows Services says to stop, this will analyze the command and if it is to Stop/Shutdown it will send a message to the receiver later in the code
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                shutdown_tx.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Shutdown => {
                shutdown_tx.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register("servicetemplate", event_handler)?;

    // Tell the system that the service is running now
    status_handle.set_service_status(running())?;

    //2 loops may not be necessary but are better for scaling up the project in some cases
    //The loops *should* be able to be deleted in the case of the service only needing to do one action when triggered, but have not tested 
    
    let _ = davids_standard_library::env::set_exe_dir();

    
    'outer: loop {


        //Do some work on repeat 
        let _ = shizuku_companion::program_code::main();



        //inner loop checks to see if the sender from the event handler has sent a message to Stop/Shutdown, will timeout and continue if no message was sent
        //if a message is sent then it will tell Windows Services it is in 'Stop Pending' and break out of the loop  
        'inner: loop {
            //instead of sleeping the thread to wait before repeating work. The timeout duration could be increased.
            match shutdown_rx.recv_timeout(std::time::Duration::from_secs(5)) {
                Ok(_) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    status_handle.set_service_status(stop_pending())?;
                    break 'outer;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
            }
            break 'inner;
        }

    }


    //tell windows the service is stopped
    status_handle.set_service_status(stopped())?;

    //return status "Ok"
    Ok(())
}


fn main() -> Result<(), windows_service::Error> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start("servicetemplate", ffi_service_main)?;
    Ok(())
}

//the functions below are just used for organization and making the code read easier.
//the contents of the functions can be places where they are called and it should work just the same
//These conatin the data to tell Windows Services the current state.
fn running() -> ServiceStatus
{
    ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: std::time::Duration::default(),
        // Not used for setting status
        process_id: None,
    }
}

fn stop_pending() -> ServiceStatus
{
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::StopPending,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::NO_ERROR,
        checkpoint: 0,
        wait_hint: std::time::Duration::default(),
        process_id: None,
    }
}

fn stopped() -> ServiceStatus
{
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: std::time::Duration::default(),
        process_id: None,
    }
}