import React, { useState, useEffect } from 'react';
import axios from 'axios';

const Dashboard = () => {
  const [analyticsData, setAnalyticsData] = useState({});
  const [deviceHealth, setDeviceHealth] = useState({});
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchAnalyticsData = async () => {
      try {
        const response = await axios.get('/api/analytics');
        setAnalyticsData(response.data);
      } catch (error) {
        console.error('Error fetching analytics data:', error);
      }
    };

    const fetchDeviceHealth = async () => {
      try {
        const response = await axios.get('/api/monitor');
        setDeviceHealth(response.data);
      } catch (error) {
        console.error('Error fetching device health:', error);
      }
    };

    fetchAnalyticsData();
    fetchDeviceHealth();
    setLoading(false);
  }, []);

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <h1>Dashboard</h1>
      <div>
        <h2>Analytics Data</h2>
        <ul>
          {Object.entries(analyticsData).map(([deviceId, count]) => (
            <li key={deviceId}>
              {deviceId}: {count} updates
            </li>
          ))}
        </ul>
      </div>
      <div>
        <h2>Device Health</h2>
        <ul>
          {Object.entries(deviceHealth).map(([deviceId, health]) => (
            <li key={deviceId}>
              {deviceId}: {health.is_online ? 'Online' : 'Offline'} (Last update: {health.last_update})
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default Dashboard;
