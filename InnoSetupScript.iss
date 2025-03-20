[Setup]
AppName=KeyboardAppLCammish
AppVersion=1.0
DefaultDirName={pf}\KeyboardAppLCammish
DefaultGroupName=KeyboardAppLCammish
OutputDir=.
OutputBaseFilename=KeyboardAppLCammish_Installer
Compression=lzma
SolidCompression=yes

[Files]
Source: "C:\Users\L.J.Cammish\Documents\assessment - logan cammish\target\release\KeyboardAppLCammish.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\L.J.Cammish\Documents\assessment - logan cammish\assets\*"; DestDir: "{app}\assets"; Flags: recursesubdirs createallsubdirs

[Icons]
Name: "{group}\KeyboardAppLCammish"; Filename: "{app}\KeyboardAppLCammish.exe"
Name: "{commondesktop}\KeyboardAppLCammish"; Filename: "{app}\KeyboardAppLCammish.exe"

[Run]
Filename: "{app}\KeyboardAppLCammish.exe"; Description: "Launch KeyboardAppLCammish"; Flags: nowait postinstall skipifsilent
