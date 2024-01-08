import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import axios from 'axios';
import App from './app';
import Dashboard from './Dashboard';
import DeviceList from './DeviceList';

jest.mock('axios');

describe('IoT Analytics Dashboard', () => {
  test('loads and displays the device list', async () => {
    const devices = [
      { id: 'device1', name: 'Temperature Sensor', status: 'active', lastActive: '2021-04-01T12:00:00Z' },
      { id: 'device2', name: 'Humidity Sensor', status: 'inactive', lastActive: '2021-04-01T11:00:00Z' },
    ];

    axios.get.mockResolvedValueOnce({ data: devices });

    render(<DeviceList />);

    expect(screen.getByText(/loading/i)).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Temperature Sensor')).toBeInTheDocument();
      expect(screen.getByText('Humidity Sensor')).toBeInTheDocument();
    });
  });

  test('loads and displays the dashboard with analytics and device health', async () => {
    const analyticsData = {
      'device1': 10,
      'device2': 5,
    };

    const deviceHealth = {
      'device1': { is_online: true, last_update: '2021-04-01T12:00:00Z' },
      'device2': { is_online: false, last_update: '2021-04-01T11:00:00Z' },
    };

    axios.get.mockResolvedValueOnce({ data: analyticsData });
    axios.get.mockResolvedValueOnce({ data: deviceHealth });

    render(<Dashboard />);

    expect(screen.getByText(/loading/i)).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('device1: 10 updates')).toBeInTheDocument();
      expect(screen.getByText('device2: 5 updates')).toBeInTheDocument();
      expect(screen.getByText('device1: Online (Last update: 2021-04-01T12:00:00Z)')).toBeInTheDocument();
      expect(screen.getByText('device2: Offline (Last update: 2021-04-01T11:00:00Z)')).toBeInTheDocument();
    });
  });

  test('selects a device and displays its data on the dashboard', async () => {
    const devices = [
      { id: 'device1', name: 'Temperature Sensor', status: 'active', lastActive: '2021-04-01T12:00:00Z' },
      { id: 'device2', name: 'Humidity Sensor', status: 'inactive', lastActive: '2021-04-01T11:00:00Z' },
    ];

    const deviceData = {
      id: 'device1',
      name: 'Temperature Sensor',
      readings: [{ timestamp: '2021-04-01T12:00:00Z', value: '23°C' }],
    };

    axios.get.mockResolvedValueOnce({ data: devices });
    axios.get.mockResolvedValueOnce({ data: deviceData });

    render(<App />);

    await waitFor(() => {
      expect(screen.getByText('Temperature Sensor')).toBeInTheDocument();
    });

    userEvent.click(screen.getByText('Temperature Sensor'));

    await waitFor(() => {
      expect(screen.getByText('Dashboard')).toBeInTheDocument();
      expect(screen.getByText('23°C')).toBeInTheDocument();
    });
  });
});
