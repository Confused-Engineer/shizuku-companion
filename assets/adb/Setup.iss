; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "shizuku"
#define MyAppVersion "1.5"
#define MyAppPublisher "Confused Engineer"
#define MyAppExeName "shizuku.exe"


[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{AA0C82A2-16D1-47A0-BDC3-7D143960BE8E}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
OutputBaseFilename=shizuku_setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
SignTool=signtool $f

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"


[Files]
Source: "adb.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "AdbWinApi.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "AdbWinUsbApi.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "shizuku.exe"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[run]
Filename: {sys}\sc.exe; Parameters: "create shizuku start= auto binPath= ""{app}\{#MyAppExeName}""" ; Flags: runhidden
Filename: {sys}\sc.exe; Parameters: "start shizuku"; Flags: runhidden

[UninstallRun]
Filename: {sys}\sc.exe; Parameters: "stop shizuku" ; Flags: runhidden ; RunOnceId: "stopsvc" 
Filename: {sys}\sc.exe; Parameters: "delete shizuku" ; Flags: runhidden ; RunOnceId: "rmvsvc"
Filename: {app}\adb.exe; Parameters: "kill-server" ; Flags: runhidden ; RunOnceId: "rmvadb"

[UninstallDelete]
Type: files; Name: "{app}\devices"
