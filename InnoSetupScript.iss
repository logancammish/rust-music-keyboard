[Setup]
AppName=KeyboardAppLCammish
AppVersion=0.3.0
DefaultDirName={pf}\KeyboardAppLCammish
DefaultGroupName=KeyboardAppLCammish
OutputDir=.
OutputBaseFilename=KeyboardAppLCammish_Installer-Windows-x86_64
Compression=lzma
SolidCompression=yes

[Files]
Source: "C:\Users\L.J.Cammish\OneDrive - Saint Kentigern\Documents\rust-music-keyboard\target\release\KeyboardAppLCammish.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "C:\Users\L.J.Cammish\OneDrive - Saint Kentigern\Documents\rust-music-keyboard\assets\*"; DestDir: "{app}\assets"; Flags: recursesubdirs createallsubdirs

[Icons]
Name: "{group}\KeyboardAppLCammish"; Filename: "{app}\KeyboardAppLCammish.exe"
Name: "{commondesktop}\KeyboardAppLCammish"; Filename: "{app}\KeyboardAppLCammish.exe"

[Run]
Filename: "{app}\KeyboardAppLCammish.exe"; Description: "Launch KeyboardAppLCammish"; Flags: nowait postinstall skipifsilent
