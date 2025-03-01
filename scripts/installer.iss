[Setup]
AppName=LabKit 
AppVersion=0.1.0
DefaultDirName={pf}\LabKit 
DefaultGroupName=EnderDreams
OutputDir=..\output
OutputBaseFilename=Lab-Kit-setup

[Files]
Source: "G:\CodeWorks\github-projects\IUT-Lab-Kit\target\release\lab.exe"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: string; ValueName: "Path"; ValueData: "{olddata};{app}"; Flags: uninsdeletevalue
