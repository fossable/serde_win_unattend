//! Rust serde bindings for Microsoft Windows Autounattend.xml.
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};

/// Represents an unattend.xml file.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "unattend")]
pub struct UnattendXml {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    pub settings: Vec<Settings>,
}

impl UnattendXml {
    pub fn write_to(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        std::fs::write(
            path.join("Autounattend.xml"),
            quick_xml::se::to_string(&self).unwrap(),
        )
        .unwrap();
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "settings")]
pub struct Settings {
    pub component: Vec<Component>,
    #[serde(rename = "@pass")]
    pub pass: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "component")]
pub struct Component {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ComputerName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiskConfiguration: Option<DiskConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub FirstLogonCommands: Option<FirstLogonCommands>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ImageInstall: Option<ImageInstall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InputLocale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SetupUILanguage: Option<SetupUILanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SystemLocale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UILanguage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UILanguageFallback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UserAccounts: Option<Vec<LocalAccounts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UserData: Option<UserData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UserLocale: Option<String>,
    #[serde(rename = "@language")]
    pub language: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@processorArchitecture")]
    pub processorArchitecture: String,
    #[serde(rename = "@publicKeyToken")]
    pub publicKeyToken: String,
    #[serde(rename = "@versionScope")]
    pub versionScope: String,
    #[serde(rename = "@xmlns:wcm")]
    pub xmlns_wcm: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
}

impl Default for Component {
    fn default() -> Self {
        Component {
            ComputerName: None,
            DiskConfiguration: None,
            FirstLogonCommands: None,
            ImageInstall: None,
            InputLocale: None,
            SetupUILanguage: None,
            SystemLocale: None,
            UILanguage: None,
            UILanguageFallback: None,
            UserAccounts: None,
            UserData: None,
            UserLocale: None,
            language: "neutral".into(),
            name: "".into(),
            processorArchitecture: "amd64".into(),
            publicKeyToken: "31bf3856ad364e35".into(),
            versionScope: "nonSxS".into(),
            xmlns_wcm: "http://schemas.microsoft.com/WMIConfig/2002/State".into(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetupUILanguage {
    pub UILanguage: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    pub AcceptEula: String,
    pub FullName: String,
    pub Organization: String,
    pub ProductKey: ProductKey,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductKey {
    pub Key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub WillShowUI: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FirstLogonCommands {
    pub SynchronousCommand: Vec<SynchronousCommand>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SynchronousCommand {
    pub CommandLine: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Description: Option<String>,
    pub Order: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RequiresUserInput: Option<String>,
    #[serde(rename = "@wcm:action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LocalAccounts {
    // pub LocalAccounts: Vec<LocalAccount>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LocalAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DisplayName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Group: Option<String>,
    pub Name: String,
    pub Password: Password,
    #[serde(rename = "@wcm:action")]
    pub action: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Password {
    pub PlainText: String,
    pub Value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiskConfiguration {
    pub Disk: Disk,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub WillShowUI: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Disk {
    pub CreatePartitions: CreatePartitions,
    pub DiskID: String,
    pub ModifyPartitions: ModifyPartitions,
    pub WillWipeDisk: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePartitions {
    pub CreatePartition: Vec<CreatePartition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePartition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Extend: Option<String>,
    pub Order: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Size: Option<String>,
    pub Type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyPartitions {
    pub ModifyPartition: Vec<ModifyPartition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyPartition {
    pub Format: String,
    pub Label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Letter: Option<String>,
    pub Order: String,
    pub PartitionID: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageInstall {
    pub OSImage: OSImage,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OSImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstallTo: Option<InstallTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstallToAvailablePartition: Option<String>,
    // pub InstallFrom: Option<InstallFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub WillShowUI: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstallTo {
    pub DiskID: String,
    pub PartitionID: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_1() {
        let raw_xml = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <unattend xmlns="urn:schemas-microsoft-com:unattend">
              <settings pass="windowsPE">
                <component name="Microsoft-Windows-International-Core-WinPE" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <!-- Windows 10 English: en-US -->
                  <!-- Windows 10 English International: en-GB -->
                  <UILanguage>en-GB</UILanguage>
                  <InputLocale>en-AU</InputLocale>
                  <SystemLocale>en-AU</SystemLocale>
                  <UserLocale>en-AU</UserLocale>
                  <SetupUILanguage>
                    <UILanguage>en-GB</UILanguage>
                  </SetupUILanguage>
                </component>
                <component name="Microsoft-Windows-Setup" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <UserData>
                    <ProductKey>
                      <Key>12345-12345-12345-12345-12345</Key>
                    </ProductKey>
                    <AcceptEula>true</AcceptEula>
                    <FullName>Full Name</FullName>
                    <Organization>Organization</Organization>
                  </UserData>
                  <ImageInstall>
                    <OSImage>
                      <InstallFrom>
                        <MetaData wcm:action="add">
                          <Key>/IMAGE/DESCRIPTION</Key>
                          <Value>Windows 10 Home</Value>
                        </MetaData>
                      </InstallFrom>
                    </OSImage>
                  </ImageInstall>
                </component>
                <component name="Microsoft-Windows-PnpCustomizationsWinPE" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <DriverPaths>
                    <PathAndCredentials wcm:keyValue="1" wcm:action="add">
                      <Path>C:\Drivers</Path>
                    </PathAndCredentials>
                    <PathAndCredentials wcm:keyValue="2" wcm:action="add">
                      <Path>D:\Drivers</Path>
                    </PathAndCredentials>
                    <PathAndCredentials wcm:keyValue="3" wcm:action="add">
                      <Path>E:\Drivers</Path>
                    </PathAndCredentials>
                    <PathAndCredentials wcm:keyValue="4" wcm:action="add">
                      <Path>X:\Drivers</Path>
                    </PathAndCredentials>
                  </DriverPaths>
                </component>
              </settings>
              <settings pass="oobeSystem">
                <component name="Microsoft-Windows-International-Core" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <InputLocale>en-AU</InputLocale>
                  <SystemLocale>en-AU</SystemLocale>
                  <UserLocale>en-AU</UserLocale>
                  <UILanguage>en-AU</UILanguage>
                  <UILanguageFallback>en-GB</UILanguageFallback>
                </component>
                <component name="Microsoft-Windows-Shell-Setup" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <UserAccounts>
                    <LocalAccounts>
                      <LocalAccount wcm:action="add">
                        <Name>admin</Name>
                        <Group>Administrators</Group>
                        <Password>
                          <Value>1234</Value>
                          <PlainText>true</PlainText>
                        </Password>
                      </LocalAccount>
                      <LocalAccount wcm:action="add">
                        <DisplayName>Full Name</DisplayName>
                        <Name>user</Name>
                        <Group>Administrators</Group>
                        <Password>
                          <Value>1234</Value>
                          <PlainText>true</PlainText>
                        </Password>
                      </LocalAccount>
                      <LocalAccount wcm:action="add">
                        <DisplayName>Other user</DisplayName>
                        <Name>user2</Name>
                        <Password>
                          <Value>1234</Value>
                          <PlainText>true</PlainText>
                        </Password>
                      </LocalAccount>
                    </LocalAccounts>
                  </UserAccounts>
                  <OOBE>
                    <HideOnlineAccountScreens>true</HideOnlineAccountScreens>
                    <HideEULAPage>true</HideEULAPage>
                    <HideWirelessSetupInOOBE>true</HideWirelessSetupInOOBE>
                    <ProtectYourPC>3</ProtectYourPC>
                    <HideLocalAccountScreen>true</HideLocalAccountScreen>
                    <HideOEMRegistrationScreen>true</HideOEMRegistrationScreen>
                    <SkipUserOOBE>true</SkipUserOOBE>
                  </OOBE>
                  <RegisteredOwner>Full name</RegisteredOwner>
                  <RegisteredOrganization>Organization</RegisteredOrganization>
                  <TimeZone>AUS Eastern Standard Time</TimeZone>
                </component>
              </settings>
              <settings pass="specialize">
                <component name="Microsoft-Windows-Deployment" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                  <RunSynchronous>
                    <RunSynchronousCommand wcm:action="add">
                      <Order>1</Order>
                      <Description>Set the product key, configure Wi-Fi, install software, apply default settings</Description>
                      <Path>CMD /C FOR %i IN (C D E X) DO IF EXIST %i:\Autounattend.xml (CALL %i:\Unattended\Unattended.cmd &amp; EXIT /B %ERRORLEVEL%)</Path>
                    </RunSynchronousCommand>
                    <RunSynchronousCommand wcm:action="add">
                      <Order>2</Order>
                      <Description>Install TightVNC Server</Description>
                      <Path>CMD /C FOR %i IN (C D E X) DO IF EXIST %i:\Autounattend.xml (CALL %i:\Unattended\UnattendedTightVNC.cmd "nZ4yUJ3O" "Shabbyr=" &amp; EXIT /B %ERRORLEVEL%)</Path>
                    </RunSynchronousCommand>
                  </RunSynchronous>
                </component>
              </settings>
              <cpi:offlineImage cpi:source="catalog://doo/lina/sources/install_windows 10 home.clg" xmlns:cpi="urn:schemas-microsoft-com:cpi" />
            </unattend>
        "#;

        let _: UnattendXml = quick_xml::de::from_str(raw_xml).unwrap();
    }

    #[test]
    fn test_deserialize_2() {
        let raw_xml = r#"
          <?xml version="1.0" encoding="utf-8"?>
          <unattend xmlns="urn:schemas-microsoft-com:unattend">
            <settings pass="windowsPE">
              <component name="Microsoft-Windows-International-Core-WinPE" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <SetupUILanguage>
                  <UILanguage>en-US</UILanguage>
                </SetupUILanguage>
                <InputLocale>0c09:00000409</InputLocale>
                <SystemLocale>en-US</SystemLocale>
                <UILanguage>en-US</UILanguage>
                <UILanguageFallback>en-US</UILanguageFallback>
                <UserLocale>en-AU</UserLocale>
              </component>
              <component name="Microsoft-Windows-Setup" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <DiskConfiguration>
                  <Disk wcm:action="add">
                    <CreatePartitions>
                      <CreatePartition wcm:action="add">
                        <Order>1</Order>
                        <Type>Primary</Type>
                        <Size>100</Size>
                      </CreatePartition>
                      <CreatePartition wcm:action="add">
                        <Extend>true</Extend>
                        <Order>2</Order>
                        <Type>Primary</Type>
                      </CreatePartition>
                    </CreatePartitions>
                    <ModifyPartitions>
                      <ModifyPartition wcm:action="add">
                        <Active>true</Active>
                        <Format>NTFS</Format>
                        <Label>System Reserved</Label>
                        <Order>1</Order>
                        <PartitionID>1</PartitionID>
                        <TypeID>0x27</TypeID>
                      </ModifyPartition>
                      <ModifyPartition wcm:action="add">
                        <Active>true</Active>
                        <Format>NTFS</Format>
                        <Label>OS</Label>
                        <Letter>C</Letter>
                        <Order>2</Order>
                        <PartitionID>2</PartitionID>
                      </ModifyPartition>
                    </ModifyPartitions>
                    <DiskID>0</DiskID>
                    <WillWipeDisk>true</WillWipeDisk>
                  </Disk>
                </DiskConfiguration>
                <ImageInstall>
                  <OSImage>
                    <InstallTo>
                      <DiskID>0</DiskID>
                      <PartitionID>2</PartitionID>
                    </InstallTo>
                    <InstallToAvailablePartition>false</InstallToAvailablePartition>
                  </OSImage>
                </ImageInstall>
                <UserData>
                  <AcceptEula>true</AcceptEula>
                  <FullName>moe</FullName>
                  <Organization>Comprofix</Organization>
                  <!--
                        NOTE: Update the <Key> element to match the Key you are using. Current Key is the KMS Key designed for Installs
                        and sysprep.
                        Product Key from http://technet.microsoft.com/en-us/library/jj612867.aspx
                    -->
                    <ProductKey>
                      <Key>W269N-WFGWX-YVC9B-4J6C9-T83GX</Key>
                      <WillShowUI>Never</WillShowUI>
                    </ProductKey>
                </UserData>
                <EnableFirewall>true</EnableFirewall>
              </component>
            </settings>
            <settings pass="offlineServicing">
              <component name="Microsoft-Windows-LUA-Settings" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <EnableLUA>false</EnableLUA>
              </component>
            </settings>
            <settings pass="generalize">
              <component name="Microsoft-Windows-Security-SPP" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <SkipRearm>1</SkipRearm>
              </component>
            </settings>
            <settings pass="specialize">
              <component name="Microsoft-Windows-International-Core" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <InputLocale>0c09:00000409</InputLocale>
                <SystemLocale>en-AU</SystemLocale>
                <UILanguage>en-AU</UILanguage>
                <UILanguageFallback>en-AU</UILanguageFallback>
                <UserLocale>en-AU</UserLocale>
              </component>
              <component name="Microsoft-Windows-Security-SPP-UX" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <SkipAutoActivation>true</SkipAutoActivation>
              </component>
              <component name="Microsoft-Windows-SQMApi" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <CEIPEnabled>0</CEIPEnabled>
              </component>
              <component name="Microsoft-Windows-Shell-Setup" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <ComputerName>autobuild-pc</ComputerName>
              </component>
              <component name="Microsoft-Windows-Deployment" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <RunSynchronous>
                  <RunSynchronousCommand wcm:action="add">
                    <Description>Disable Login animation for Windows 10</Description>
                    <Order>1</Order>
                    <Path>reg add HKLM\Software\Microsoft\Windows\CurrentVersion\Policies\System /v EnableFirstLogonAnimation /t REG_DWORD /d 0 /f</Path>
                  </RunSynchronousCommand>
                  <RunSynchronousCommand wcm:action="add">
                    <Description>Disable Login animation for Windows 10</Description>
                    <Order>2</Order>
                    <Path>reg add HKLM\System\CurrentControlSet\Control\Network\NewNetworkWindowOff /F</Path>
                  </RunSynchronousCommand>
                </RunSynchronous>
              </component>
            </settings>
            <settings pass="oobeSystem">
              <component name="Microsoft-Windows-Shell-Setup" processorArchitecture="amd64" publicKeyToken="31bf3856ad364e35" language="neutral" versionScope="nonSxS" xmlns:wcm="http://schemas.microsoft.com/WMIConfig/2002/State" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                <AutoLogon>
                  <Password>
                    <Value></Value>
                    <PlainText>true</PlainText>
                  </Password>
                  <Enabled>true</Enabled>
                  <Username>moe</Username>
                </AutoLogon>
                <OOBE>
                  <HideEULAPage>true</HideEULAPage>
                  <HideOEMRegistrationScreen>true</HideOEMRegistrationScreen>
                  <HideOnlineAccountScreens>true</HideOnlineAccountScreens>
                  <HideWirelessSetupInOOBE>true</HideWirelessSetupInOOBE>
                  <NetworkLocation>Home</NetworkLocation>
                  <SkipUserOOBE>true</SkipUserOOBE>
                  <SkipMachineOOBE>true</SkipMachineOOBE>
                  <ProtectYourPC>1</ProtectYourPC>
                </OOBE>
                <UserAccounts>
                  <LocalAccounts>
                    <LocalAccount wcm:action="add">
                      <Password>
                        <Value></Value>
                        <PlainText>true</PlainText>
                      </Password>
                      <Description></Description>
                      <DisplayName>moe</DisplayName>
                      <Group>Administrators</Group>
                      <Name>moe</Name>
                    </LocalAccount>
                  </LocalAccounts>
                </UserAccounts>
                <RegisteredOrganization>Comprofix</RegisteredOrganization>
                <RegisteredOwner>moe</RegisteredOwner>
                <DisableAutoDaylightTimeSet>false</DisableAutoDaylightTimeSet>
                <FirstLogonCommands>
                  <SynchronousCommand wcm:action="add">
                    <Description>Control Panel Classic View</Description>
                    <Order>1</Order>
                    <CommandLine>reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\ControlPanel" /v StartupPage /t REG_DWORD /d 1 /f</CommandLine>
                    <RequiresUserInput>true</RequiresUserInput>
                  </SynchronousCommand>
                  <SynchronousCommand wcm:action="add">
                    <Description>Control Panel Icon Size</Description>
                    <Order>2</Order>
                    <CommandLine>reg add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\ControlPanel" /v AllItemsIconView /t REG_DWORD /d 1 /f</CommandLine>
                    <RequiresUserInput>true</RequiresUserInput>
                  </SynchronousCommand>
                  <SynchronousCommand wcm:action="add">
                    <Order>3</Order>
                    <Description>Set-ExecutionPolicy Unrestricted</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd.exe /c powershell -Command "Set-ExecutionPolicy Unrestricted"</CommandLine>
                  </SynchronousCommand>
                  <SynchronousCommand wcm:action="add">
                    <Order>4</Order>
                    <Description>Fix Network</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\fixnetwork.ps1  cmd /c powershell -file %i:\fixnetwork.ps1"</CommandLine>
                  </SynchronousCommand>

                  <SynchronousCommand wcm:action="add">
                    <Order>20</Order>
                    <Description>Disable Updates</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\disableWindowsUpdates.ps1  cmd /c powershell -file %i:\disableWindowsUpdates.ps1"</CommandLine>
                  </SynchronousCommand>

                  <!-- Install Chocolatey -->
                  <SynchronousCommand wcm:action="add">
                    <Order>50</Order>
                    <Description>Install Chocolatey</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\chocolatey.ps1  cmd /c powershell -file %i:\chocolatey.ps1"</CommandLine>
                  </SynchronousCommand>

                  <!-- Install Applications via Chocolatey -->
                  <SynchronousCommand wcm:action="add">
                    <Order>51</Order>
                    <Description>Install Apps via Chocolatey</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\chocolatey-apps.cmd  cmd /c %i:\chocolatey-apps.cmd"</CommandLine>
                  </SynchronousCommand>

                  <!-- Disable Cortana Search bar -->
                  <SynchronousCommand wcm:action="add">
                      <CommandLine>reg.exe ADD HKCU\Software\Microsoft\Windows\CurrentVersion\Search /v SearchboxTaskbarMode /t REG_DWORD /d 0</CommandLine>
                      <Order>97</Order>
                      <Description>Disable Cortana</Description>
                  </SynchronousCommand>

                  <!-- Enable UAC -->
                  <SynchronousCommand wcm:action="add">
                    <Order>98</Order>
                    <Description>Enable UAC</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\enableUAC.ps1  cmd /c powershell -file %i:\enableUAC.ps1"</CommandLine>
                  </SynchronousCommand>

                  <!-- Remove OneDrive Integration -->
                  <SynchronousCommand wcm:action="add">
                    <Order>499</Order>
                    <Description>Enable UAC</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd /q /c "FOR %i IN (A B C D E F G H I J K L N M O P Q R S T U V W X Y Z) DO IF EXIST %i:\removeOneDrive.ps1  cmd /c powershell -file %i:\removeOneDrive.ps1"</CommandLine>
                  </SynchronousCommand>

                  <!-- Restarts System so changes take effect -->
                  <SynchronousCommand wcm:action="add">
                    <Order>500</Order>
                    <Description>Restart Computer</Description>
                    <RequiresUserInput>false</RequiresUserInput>
                    <CommandLine>cmd.exe /c powershell.exe -command "restart-computer -force"</CommandLine>
                  </SynchronousCommand>

                </FirstLogonCommands>
                <TimeZone>Brisbane</TimeZone>
              </component>
            </settings>
          </unattend>
        "#;

        let _: UnattendXml = quick_xml::de::from_str(raw_xml).unwrap();
    }
}
