if (Test-Path "D:\openbattlesim\core\obtl\target\debug\libobtl.so") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libobtl.so") {
		Remove-Item D:\openbattlesim\core\lib\d_libobtl.so
}
	Move-Item -Path C:\openbattlesim\core\obtl\target\debug\libobtl.so -Destination D:\openbattlesim\core\lib\d_libobtl.so
}

if (Test-Path "D:\openbattlesim\core\obtl\target\release\libobtl.so") {
	if (Test-Path "D:\openbattlesim\core\lib\libobtl.so") {
		Remove-Item D:\openbattlesim\core\lib\libobtl.so
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\release\libobtl.so -Destination D:\openbattlesim\core\lib\libobtl.so
}

if (Test-Path "D:\openbattlesim\core\obtl\target\debug\obtl.dll") {
	if (Test-Path "D:\openbattlesim\core\lib\d_obtl.dll") {
		Remove-Item D:\openbattlesim\core\lib\d_obtl.dll
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\debug\obtl.dll -Destination D:\openbattlesim\core\lib\d_obtl.dll
}

if (Test-Path "D:\openbattlesim\core\obtl\target\release\obtl.dll") {
	if (Test-Path "D:\openbattlesim\core\lib\obtl.dll") {
		Remove-Item D:\openbattlesim\core\lib\obtl.dll
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\release\obtl.dll -Destination D:\openbattlesim\core\lib\obtl.dll
}

if (Test-Path "D:\openbattlesim\core\obtl\target\debug\libotbtl.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libobtl.dylib") {
		Remove-Item D:\openbattlesim\core\lib\d_libobtl.dylib
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\debug\libotbtl.dylib -Destination D:\openbattlesim\core\lib\d_libobtl.dylib
}

if (Test-Path "D:\openbattlesim\core\obtl\target\release\libotbtl.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\libobtl.dylib") {
		Remove-Item D:\openbattlesim\core\lib\libobtl.dylib
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\release\libotbtl.dylib -Destination D:\openbattlesim\core\lib\libobtl.dylib
}

if (Test-Path "D:\openbattlesim\core\obtl\target\debug\libobtl.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libobtl.dylib") {
		Remove-Item D:\openbattlesim\core\lib\d_libobtl.dylib
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\debug\libotbtl.dylib -Destination D:\openbattlesim\core\lib\d_libobtl.dylib
}

if (Test-Path "D:\openbattlesim\core\obtl\target\release\libobtl.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\libobtl.dylib") {
		Remove-Item D:\openbattlesim\core\lib\libobtl.dylib
}
	Move-Item -Path D:\openbattlesim\core\obtl\target\release\libobtl.dylib -Destination D:\openbattlesim\core\lib\libobtl.dylib
}



if (Test-Path "D:\openbattlesim\core\maploader\target\debug\libmaploader.so") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libmaploader.so") {
		Remove-Item D:\openbattlesim\core\lib\d_libmaploader.so
	}
	Move-Item -Path C:\openbattlesim\core\maploader\target\debug\libmaploader.so -Destination D:\openbattlesim\core\lib\d_libmaploader.so
}

if (Test-Path "D:\openbattlesim\core\maploader\target\release\libmaploader.so") {
	if (Test-Path "D:\openbattlesim\core\lib\libmaploader.so") {
		Remove-Item D:\openbattlesim\core\lib\libmaploader.so
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\release\libmaploader.so -Destination D:\openbattlesim\core\lib\libmaploader.so
}

if (Test-Path "D:\openbattlesim\core\maploader\target\debug\maploader.dll") {
	if (Test-Path "D:\openbattlesim\core\lib\d_maploader.dll") {
		Remove-Item D:\openbattlesim\core\lib\d_maploader.dll
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\debug\maploader.dll -Destination D:\openbattlesim\core\lib\d_maploader.dll
}

if (Test-Path "D:\openbattlesim\core\maploader\target\release\maploader.dll") {
	if (Test-Path "D:\openbattlesim\core\lib\maploader.dll") {
		Remove-Item D:\openbattlesim\core\lib\maploader.dll
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\release\maploader.dll -Destination D:\openbattlesim\core\lib\maploader.dll
}

if (Test-Path "D:\openbattlesim\core\maploader\target\debug\libmaploader.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libmaploader.dylib") {
		Remove-Item D:\openbattlesim\core\lib\d_libmaploader.dylib
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\debug\libmaploader.dylib -Destination D:\openbattlesim\core\lib\d_libmaploader.dylib
}

if (Test-Path "D:\openbattlesim\core\maploader\target\release\libmaploader.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\libmaploader.dylib") {
		Remove-Item D:\openbattlesim\core\lib\libmaploader.dylib
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\release\libmaploader.dylib -Destination D:\openbattlesim\core\lib\libmaploader.dylib
}

if (Test-Path "D:\openbattlesim\core\maploader\target\debug\libmaploader.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\d_libmaploader.dylib") {
		Remove-Item D:\openbattlesim\core\lib\d_libmaploader.dylib
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\debug\libmaploader.dylib -Destination D:\openbattlesim\core\lib\d_libmaploader.dylib
}

if (Test-Path "D:\openbattlesim\core\maploader\target\release\libmaploader.dylib") {
	if (Test-Path "D:\openbattlesim\core\lib\libmaploader.dylib") {
		Remove-Item D:\openbattlesim\core\lib\libmaploader.dylib
	}
	Move-Item -Path D:\openbattlesim\core\maploader\target\release\libmaploader.dylib -Destination D:\openbattlesim\core\lib\libmaploader.dylib
}