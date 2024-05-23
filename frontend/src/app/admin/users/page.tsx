'use client';

import { useEffect, useState } from 'react';
import UserTable from './UserTable';

export default function Home() {
    const [users, setUsers] = useState([]);

    useEffect(() => {
        const fetchConfigData = async () => {
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/superuser/users`, {
                    method: 'GET',
                    headers,
                });

                console.log(response);

                if (response.ok) {
                    const data = await response.json();
                    setUsers(data);
                } else if (response.status === 401) {
                    // Handle authorization error
                    console.error('Invalid username or password');
                    localStorage.removeItem('username');
                    localStorage.removeItem('password');
                } else {
                    console.error('Failed to fetch user data');
                }
            } catch (error) {
                console.error('Error fetching user data:', error);
            }
        };

        fetchConfigData();
    }, []);

    return (
        <UserTable users={users} />
    );
}
