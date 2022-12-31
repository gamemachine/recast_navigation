cmake_minimum_required(VERSION 3.15.0 FATAL_ERROR)

set(CMAKE_SYSTEM_VERSION 10.0 CACHE STRING "" FORCE)

project(AiNav CXX)

################################################################################
# Set target arch type if empty. Visual studio solution generator provides it.
################################################################################
if(NOT CMAKE_VS_PLATFORM_NAME)
    set(CMAKE_VS_PLATFORM_NAME "x64")
endif()
message("${CMAKE_VS_PLATFORM_NAME} architecture in use")

if(NOT ("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64"
     OR "${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86"))
    message(FATAL_ERROR "${CMAKE_VS_PLATFORM_NAME} arch is not supported!")
endif()

################################################################################
# Global configuration types
################################################################################
set(CMAKE_CONFIGURATION_TYPES
    "Debug"
    "Release"
    CACHE STRING "" FORCE
)

################################################################################
# Global compiler options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_CXX_FLAGS "")
    set(CMAKE_CXX_FLAGS_DEBUG "")
    set(CMAKE_CXX_FLAGS_RELEASE "")
endif()

################################################################################
# Global linker options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_EXE_LINKER_FLAGS "")
    set(CMAKE_MODULE_LINKER_FLAGS "")
    set(CMAKE_SHARED_LINKER_FLAGS "")
    set(CMAKE_STATIC_LINKER_FLAGS "")
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_DEBUG "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUG "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_DEBUG "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_RELEASE "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_RELEASE "${CMAKE_STATIC_LINKER_FLAGS}")
endif()

################################################################################
# Nuget packages function stub.
################################################################################
function(use_package TARGET PACKAGE VERSION)
    message(WARNING "No implementation of use_package. Create yours. "
                    "Package \"${PACKAGE}\" with version \"${VERSION}\" "
                    "for target \"${TARGET}\" is ignored!")
endfunction()

################################################################################
# Common utils
################################################################################
include(CMake/Utils.cmake)

################################################################################
# Additional Global Settings(add specific info there)
################################################################################
include(CMake/GlobalSettingsInclude.cmake OPTIONAL)

################################################################################
# Use solution folders feature
################################################################################
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

################################################################################
# Sub-projects
################################################################################



























set(PROJECT_NAME AiNav)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "AiCrowd.hpp"
    "AiNav.h"
    "AiQuery.hpp"
    "Detour/Include/DetourAlloc.h"
    "Detour/Include/DetourAssert.h"
    "Detour/Include/DetourCommon.h"
    "Detour/Include/DetourMath.h"
    "Detour/Include/DetourNavMesh.h"
    "Detour/Include/DetourNavMeshBuilder.h"
    "Detour/Include/DetourNavMeshQuery.h"
    "Detour/Include/DetourNode.h"
    "Detour/Include/DetourStatus.h"
    "DetourCrowd/Include/DetourCrowd.h"
    "DetourCrowd/Include/DetourLocalBoundary.h"
    "DetourCrowd/Include/DetourObstacleAvoidance.h"
    "DetourCrowd/Include/DetourPathCorridor.h"
    "DetourCrowd/Include/DetourPathQueue.h"
    "DetourCrowd/Include/DetourProximityGrid.h"
    "Navigation.hpp"
    "NavigationBuilder.hpp"
    "NavigationMesh.hpp"
    "Recast/Include/Recast.h"
    "Recast/Include/RecastAlloc.h"
    "Recast/Include/RecastAssert.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Source_Files
    "AiCrowd.cpp"
    "AiNav.cpp"
    "AiQuery.cpp"
    "Detour/Source/DetourAlloc.cpp"
    "Detour/Source/DetourAssert.cpp"
    "Detour/Source/DetourCommon.cpp"
    "Detour/Source/DetourNavMesh.cpp"
    "Detour/Source/DetourNavMeshBuilder.cpp"
    "Detour/Source/DetourNavMeshQuery.cpp"
    "Detour/Source/DetourNode.cpp"
    "DetourCrowd/Source/DetourCrowd.cpp"
    "DetourCrowd/Source/DetourLocalBoundary.cpp"
    "DetourCrowd/Source/DetourObstacleAvoidance.cpp"
    "DetourCrowd/Source/DetourPathCorridor.cpp"
    "DetourCrowd/Source/DetourPathQueue.cpp"
    "DetourCrowd/Source/DetourProximityGrid.cpp"
    "dllmain.cpp"
    "NavigationBuilder.cpp"
    "NavigationMesh.cpp"
    "Recast/Source/Recast.cpp"
    "Recast/Source/RecastAlloc.cpp"
    "Recast/Source/RecastArea.cpp"
    "Recast/Source/RecastAssert.cpp"
    "Recast/Source/RecastContour.cpp"
    "Recast/Source/RecastFilter.cpp"
    "Recast/Source/RecastLayers.cpp"
    "Recast/Source/RecastMesh.cpp"
    "Recast/Source/RecastMeshDetail.cpp"
    "Recast/Source/RecastRasterization.cpp"
    "Recast/Source/RecastRegion.cpp"
)
source_group("Source Files" FILES ${Source_Files})

set(ALL_FILES
    ${Header_Files}
    ${Source_Files}
)

################################################################################
# Target
################################################################################
add_library(${PROJECT_NAME} SHARED ${ALL_FILES})

#add_precompiled_header(${PROJECT_NAME} "pch.h" ".")

use_props(${PROJECT_NAME} "${CMAKE_CONFIGURATION_TYPES}" "${DEFAULT_CXX_PROPS}")
set(ROOT_NAMESPACE AiNav)

set_target_properties(${PROJECT_NAME} PROPERTIES
    VS_GLOBAL_KEYWORD "Win32Proj"
)
if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64")
    set_target_properties(${PROJECT_NAME} PROPERTIES
        INTERPROCEDURAL_OPTIMIZATION_RELEASE "TRUE"
    )
elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86")
    set_target_properties(${PROJECT_NAME} PROPERTIES
        INTERPROCEDURAL_OPTIMIZATION_RELEASE "TRUE"
    )
endif()
################################################################################
# Include directories
################################################################################
if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64")
    target_include_directories(${PROJECT_NAME} PUBLIC
        "${CMAKE_CURRENT_SOURCE_DIR}/DetourCrowd/Source;"
        "${CMAKE_CURRENT_SOURCE_DIR}/DetourCrowd/Include;"
        "${CMAKE_CURRENT_SOURCE_DIR}/Detour/Source;"
        "${CMAKE_CURRENT_SOURCE_DIR}/Detour/Include;"
        "${CMAKE_CURRENT_SOURCE_DIR}/Recast/Source;"
        "${CMAKE_CURRENT_SOURCE_DIR}/Recast/Include"
    )
endif()

################################################################################
# Compile definitions
################################################################################
if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64")
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        "$<$<CONFIG:Debug>:"
            "_DEBUG"
        ">"
        "$<$<CONFIG:Release>:"
            "NDEBUG"
        ">"
        "AINAV_EXPORTS;"
        "_WINDOWS;"
        "_USRDLL;"
        "UNICODE;"
        "_UNICODE"
    )
elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86")
    target_compile_definitions(${PROJECT_NAME} PRIVATE
        "$<$<CONFIG:Debug>:"
            "_DEBUG"
        ">"
        "$<$<CONFIG:Release>:"
            "NDEBUG"
        ">"
        "WIN32;"
        "AINAV_EXPORTS;"
        "_WINDOWS;"
        "_USRDLL;"
        "UNICODE;"
        "_UNICODE"
    )
endif()

################################################################################
# Compile and link options
################################################################################
if(MSVC)
    if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64")
        target_compile_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
                /Od
            >
            $<$<CONFIG:Release>:
                /O2;
                /Oi;
                /Gy;
                /Ot
            >
            /permissive-;
            /sdl;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            ${DEFAULT_CXX_EXCEPTION_HANDLING};
            /Y-
        )
    elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86")
        target_compile_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
                /Od
            >
            $<$<CONFIG:Release>:
                /O2;
                /Oi;
                /Gy
            >
            /permissive-;
            /sdl;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            ${DEFAULT_CXX_EXCEPTION_HANDLING}
        )
    endif()
    if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x64")
        target_link_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
                /INCREMENTAL
            >
            $<$<CONFIG:Release>:
                /OPT:REF;
                /OPT:ICF;
                /INCREMENTAL:NO
            >
            /DEBUG;
            /SUBSYSTEM:WINDOWS
        )
    elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86")
        target_link_options(${PROJECT_NAME} PRIVATE
            $<$<CONFIG:Debug>:
                /INCREMENTAL
            >
            $<$<CONFIG:Release>:
                /OPT:REF;
                /OPT:ICF;
                /INCREMENTAL:NO
            >
            /DEBUG;
            /SUBSYSTEM:WINDOWS
        )
    endif()
endif()

