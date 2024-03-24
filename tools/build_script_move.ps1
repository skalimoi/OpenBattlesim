if (Test-Path "C:\openbattlesim\core\obtl\target\debug\libobtl.so") {
	if (Test-Path "C:\openbattlesim\core\lib\d_libobtl.so") {
		Remove-Item C:\openbattlesim\core\lib\d_libobtl.so
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\debug\libobtl.so -Destination C:\openbattlesim\core\lib\d_libobtl.so
}

if (Test-Path "C:\openbattlesim\core\obtl\target\release\libobtl.so") {
	if (Test-Path "C:\openbattlesim\core\lib\libobtl.so") {
		Remove-Item C:\openbattlesim\core\lib\libobtl.so
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\release\libobtl.so -Destination C:\openbattlesim\core\lib\libobtl.so
}

if (Test-Path "C:\openbattlesim\core\obtl\target\debug\obtl.dll") {
	if (Test-Path "C:\openbattlesim\core\lib\d_obtl.dll") {
		Remove-Item C:\openbattlesim\core\lib\d_obtl.dll
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\debug\obtl.dll -Destination C:\openbattlesim\core\lib\d_obtl.dll
}

if (Test-Path "C:\openbattlesim\core\obtl\target\release\obtl.dll") {
	if (Test-Path "C:\openbattlesim\core\lib\obtl.dll") {
		Remove-Item C:\openbattlesim\core\lib\obtl.dll
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\release\obtl.dll -Destination C:\openbattlesim\core\lib\obtl.dll
}

if (Test-Path "C:\openbattlesim\core\obtl\target\debug\libotbtl.dylib") {
	if (Test-Path "C:\openbattlesim\core\lib\d_libobtl.dylib") {
		Remove-Item C:\openbattlesim\core\lib\d_libobtl.dylib
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\debug\libotbtl.dylib -Destination C:\openbattlesim\core\lib\d_libobtl.dylib
}

if (Test-Path "C:\openbattlesim\core\obtl\target\release\libotbtl.dylib") {
	if (Test-Path "C:\openbattlesim\core\lib\libobtl.dylib") {
		Remove-Item C:\openbattlesim\core\lib\libobtl.dylib
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\release\libotbtl.dylib -Destination C:\openbattlesim\core\lib\libobtl.dylib
}

if (Test-Path "C:\openbattlesim\core\obtl\target\debug\libotbtl.dylib") {
	if (Test-Path "C:\openbattlesim\core\lib\d_libobtl.dylib") {
		Remove-Item C:\openbattlesim\core\lib\d_libobtl.dylib
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\debug\libotbtl.dylib -Destination C:\openbattlesim\core\lib\d_libobtl.dylib
}

if (Test-Path "C:\openbattlesim\core\obtl\target\release\libotbtl.dylib") {
	if (Test-Path "C:\openbattlesim\core\lib\libobtl.dylib") {
		Remove-Item C:\openbattlesim\core\lib\libobtl.dylib
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\release\libotbtl.dylib -Destination C:\openbattlesim\core\lib\libobtl.dylib
}