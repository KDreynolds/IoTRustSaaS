import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Dashboard from './Dashboard';
import DeviceList from './DeviceList';

const App = () => {
  const [devices, setDevices] = useState([]);
  const [selectedDevice, setSelectedDevice] = useState(null);
  const [deviceData, setDeviceData] = useState({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchDevices = async () => {
      try {
        const response = await axios.get('/api/devices');
        setDevices(response.data);
        setLoading(false);
      } catch (error) {
        console.error('Error fetching devices:', error);
      }
    };

    fetchDevices();
  }, []);

  const handleDeviceSelect = async (deviceId) => {
    setSelectedDevice(deviceId);
    try {
      const response = await axios.get(`/api/devices/${deviceId}`);
      setDeviceData(response.data);
    } catch (error) {
      console.error('Error fetching device data:', error);
    }
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div className="app-container">
      <h1>IoT Device Monitoring Dashboard</h1>
      <DeviceList devices={devices} onDeviceSelect={handleDeviceSelect} />
      {selectedDevice && <Dashboard deviceData={deviceData} />}
    </div>
  );
};

export default App;
