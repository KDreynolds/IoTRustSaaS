import React, { useState, useEffect } from 'react';
import { Table } from 'antd';

// Assuming there's a service that fetches device data from the API
import { fetchDevices } from './api_service';

const DeviceList = () => {
  const [devices, setDevices] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const getDevices = async () => {
      try {
        setLoading(true);
        const response = await fetchDevices();
        setDevices(response.data);
        setLoading(false);
      } catch (error) {
        console.error('Error fetching devices:', error);
        setLoading(false);
      }
    };

    getDevices();
  }, []);

  const columns = [
    {
      title: 'Device ID',
      dataIndex: 'id',
      key: 'id',
    },
    {
      title: 'Name',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status) => (
        <span style={{ color: status === 'active' ? 'green' : 'red' }}>
          {status.toUpperCase()}
        </span>
      ),
    },
    {
      title: 'Last Active',
      dataIndex: 'lastActive',
      key: 'lastActive',
    },
  ];

  return (
    <div>
      <h2>Device List</h2>
      <Table
        dataSource={devices}
        columns={columns}
        rowKey="id"
        loading={loading}
      />
    </div>
  );
};

export default DeviceList;
