package plugins

import (
	"fmt"
	"os"

	"github.com/Wifx/gonetworkmanager"
)

const (
	ethernetType                 = "802-3-ethernet"
	ethernetSection              = "802-3-ethernet"
	ethernetSectionAutoNegotiate = "auto-negotiate"
	connectionSection            = "connection"
	connectionSectionID          = "id"
	connectionSectionType        = "type"
	connectionSectionUUID        = "uuid"
	connectionSectionIfaceName   = "interface-name"
	connectionSectionAutoconnect = "autoconnect"
	ip4Section                   = "ipv4"
	ip4SectionAddressData        = "address-data"
	ip4SectionAddresses          = "addresses"
	ip4SectionAddress            = "address"
	ip4SectionPrefix             = "prefix"
	ip4SectionMethod             = "method"
	ip4SectionGateway            = "gateway"
	ip4SectionNeverDefault       = "never-default"
	ip6Section                   = "ipv6"
	ip6SectionMethod             = "method"

	connectionID                   = "My Connection"
	interfaceName                  = "eth1"
	desiredIPAddress               = "192.168.1.1"
	desiredGatewayAddress          = "192.168.1.1"
	desiredIPAddressNumerical      = 16885952
	desiredIPPrefix                = 24
	desiredGatewayAddressNumerical = 16885952

	// Allows for static ip
	desiredIP4Method = "manual"

	// Would like this to be "disabled" however not supported
	// in the current network manager stack
	desiredIP6Method = "ignore"
)

func newNetworkManagerPluginOptions() {

	/* Create new instance of gonetworkmanager */
	nm, err := gonetworkmanager.NewNetworkManager()

	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	/* Get devices */
	devices, err := nm.GetPropertyAllDevices()
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}

	activeConnections, _ := nm.GetPropertyActiveConnections()

	for _, activeConnection := range activeConnections {
		conn, _ := activeConnection.GetPropertyConnection()
		sett, _ := conn.GetSettings()
		fmt.Println(sett["connection"])

	}

	/* Show each device path and interface name */
	for _, device := range devices {

		// activeConnection, _ := device.GetPropertyActiveConnection()
		// id, _ := activeConnection.GetPropertyID()
		// c, _ := activeConnection.GetPropertyConnection()
		// cs, _ := c.GetSettings()

		deviceInterface, _ := device.GetPropertyInterface()
		deviceType, _ := device.GetPropertyDeviceType()
		fmt.Println(deviceType)

		fmt.Println(deviceInterface + " - " + string(device.GetPath()))
		availableConnections, _ := device.GetPropertyAvailableConnections()

		for _, availableConnection := range availableConnections {
			fmt.Println(availableConnection)
			acSettings, _ := availableConnection.GetSettings()

			fmt.Println(acSettings)
		}
	}

	printExistingConnections()

}

func printExistingConnections() {
	// See if our connection already exists
	settings, _ := gonetworkmanager.NewSettings()

	currentConnections, _ := settings.ListConnections()

	for _, v := range currentConnections {
		connectionSettings, settingsError := v.GetSettings()
		if settingsError != nil {
			fmt.Println("settings error, continuing")
			continue
		}

		currentConnectionSection := connectionSettings["connection"]
		fmt.Println(currentConnectionSection)
	}
}
