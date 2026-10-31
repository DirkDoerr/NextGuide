; NextGuide Editor v1 — NSIS installer
; Expects a pre-built tree at dist\windows-bundle (see assemble-bundle.sh).
; Compile with: makensis /DPRODUCT_VERSION=0.1.0 packaging\windows\nextguide.nsi

!ifndef PRODUCT_VERSION
  !define PRODUCT_VERSION "0.1.0"
!endif
!ifndef BUNDLE_DIR
  !define BUNDLE_DIR "..\..\dist\windows-bundle"
!endif
!ifndef OUT_FILE
  !define OUT_FILE "..\..\dist\NextGuide-${PRODUCT_VERSION}-windows-x64-setup.exe"
!endif

Name "NextGuide"
OutFile "${OUT_FILE}"
InstallDir "$PROGRAMFILES64\NextGuide"
InstallDirRegKey HKLM "Software\NextGuide" "InstallDir"
RequestExecutionLevel admin
Unicode true
SetCompressor /SOLID lzma

!include "MUI2.nsh"

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "NextGuide" SecApp
  SetOutPath "$INSTDIR"
  File /r "${BUNDLE_DIR}\*.*"

  WriteRegStr HKLM "Software\NextGuide" "InstallDir" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\NextGuide" \
    "DisplayName" "NextGuide"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\NextGuide" \
    "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\NextGuide" \
    "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\NextGuide" \
    "Publisher" "NextGuide"

  CreateDirectory "$SMPROGRAMS\NextGuide"
  CreateShortCut "$SMPROGRAMS\NextGuide\NextGuide.lnk" "$INSTDIR\nextguide.exe"
  CreateShortCut "$SMPROGRAMS\NextGuide\nextguide-cli.lnk" "$INSTDIR\nextguide-cli.exe"
  CreateShortCut "$DESKTOP\NextGuide.lnk" "$INSTDIR\nextguide.exe"

  WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete "$SMPROGRAMS\NextGuide\NextGuide.lnk"
  Delete "$SMPROGRAMS\NextGuide\nextguide-cli.lnk"
  RMDir "$SMPROGRAMS\NextGuide"
  Delete "$DESKTOP\NextGuide.lnk"
  RMDir /r "$INSTDIR"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\NextGuide"
  DeleteRegKey HKLM "Software\NextGuide"
SectionEnd
