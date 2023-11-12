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
    #[serde(rename = "@pass")]
    pub pass: String,
    pub component: Vec<Component>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename = "component")]
pub struct Component {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@processorArchitecture")]
    pub processorArchitecture: String,
    #[serde(rename = "@publicKeyToken")]
    pub publicKeyToken: String,
    #[serde(rename = "@language")]
    pub language: String,
    #[serde(rename = "@versionScope")]
    pub versionScope: String,
    pub ComputerName: Option<ComputerName>,
    pub DiskConfiguration: Option<DiskConfiguration>,
    pub ImageInstall: Option<ImageInstall>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiskConfiguration {
    pub WillShowUI: WillShowUI,
    pub Disk: Disk,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Disk {
    pub CreatePartitions: CreatePartitions,
    pub ModifyPartitions: ModifyPartitions,
    pub WillWipeDisk: WillWipeDisk,
    pub DiskID: DiskID,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePartitions {
    pub CreatePartition: Vec<CreatePartition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePartition {
    pub Order: Order,
    pub Size: Size,
    pub Type: Type,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyPartitions {
    pub ModifyPartition: Vec<ModifyPartition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyPartition {
    pub Format: Format,
    pub Label: Label,
    pub Order: Order,
    pub PartitionID: PartitionID,
    pub Extend: Option<Extend>,
    pub Letter: Option<Letter>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageInstall {
    pub OSImage: OSImage,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OSImage {
    pub InstallTo: Option<InstallTo>,
    // pub InstallFrom: Option<InstallFrom>,
    pub WillShowUI: Option<WillShowUI>,
    pub InstallToAvailablePartition: Option<InstallToAvailablePartition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstallTo {
    pub DiskID: DiskID,
    pub PartitionID: PartitionID,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InstallToAvailablePartition {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Format {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PartitionID {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Letter {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Extend {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiskID {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WillWipeDisk {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Size {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Type {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WillShowUI {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ComputerName {
    #[serde(rename = "$value")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
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
}
