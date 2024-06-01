'use client'

import React, {useState, useEffect, SetStateAction} from 'react';
import UserTable from './UserTable';
import Button from "@/components/button";

export default function Home() {
    const [users, setUsers] = useState([]);
    const [guild_id, setSearchTerm] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(false);
    const [done, setDone] = useState(false);
    const [resultMessage, setResultMessage] = useState('');

    useEffect(() => {
        const fetchUserList = async () => {
            try {

                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/users`, {
                    method: 'GET',
                    headers,
                });

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

        fetchUserList();
    }, []);

    const handlePull = async () => {
        try {
            setLoading(true);

            const username = localStorage.getItem('username');
            const password = localStorage.getItem('password');

            const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/members/pull`, {
                method: 'POST',
                headers: {
                    'Authorization': `Basic ${btoa(`${username}:${password}`)}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    guild_id: guild_id,
                }),
            });

            if (response.ok) {
                setLoading(false);
                setError(false);
                setDone(true);
            } else {
                setLoading(false);y
                setError(true);

                console.error('Failed to pull users');
            }
        } catch (error) {
            setLoading(false);
            setError(true);

            console.error('Error pulling users:', error);
        } finally {
            // Reset button state after a delay
            setTimeout(() => {
                setResultMessage('');
                setLoading(false);
                setError(false);
                setDone(false);
            }, 5000);
        }
    };

    const handleInputChange = (event: { target: { value: SetStateAction<string>; }; }) => {
        setSearchTerm(event.target.value);
    };

    return (
        <div className="p-4 mt-5">
            <div className="flex items-center mb-8">
                <div className="mr-2">
                    <label htmlFor="guildid" className="sr-only">
                        GuildID
                    </label>
                    <input
                        type="text"
                        autoComplete="GuildID"
                        required
                        value={guild_id}
                        onChange={handleInputChange}
                        className="appearance-none bg-slate-800 rounded block w-full px-3 py-2 border border-gray-700 placeholder-gray-500 text-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                        placeholder="Guild ID"
                    />
                </div>
                <Button
                    loading={loading}
                    loadingClass="transition-all overflow-hidden text-white px-4 py-3 rounded-lg shadow-lg ${loading ? loadingClass : ''} ${disabled ? disabledClass : ''} ${error ? errorClass : ''} ${done ? doneClass : ''} ${loading || error || done ? 'pr-8 pl-4' : 'rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600'"
                    error={error}
                    errorClass="bg-red-600 scale-110 shake active:bg-red-600"
                    done={done}
                    doneClass="bg-green-600 scale-110 active:bg-green-600"
                    onClick={handlePull}
                >
                    {resultMessage ? resultMessage : 'Pull!'}
                </Button>
            </div>
            <UserTable users={users}/>
        </div>
    );
}
