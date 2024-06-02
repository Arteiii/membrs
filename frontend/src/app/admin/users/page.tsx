'use client';

import React, {useState, useEffect} from 'react';
import UserTable from './UserTable';
import Button from "@/components/button";
import GuildSelect from '../components/guild-select';

export default function Home() {
    const [users, setUsers] = useState([]);
    const [selectedGuild, setSelectedGuild] = useState<{
        icon: string;
        id: string;
        name: string;
    } | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(false);
    const [done, setDone] = useState(false);
    const [resultMessage, setResultMessage] = useState('');

    useEffect(() => {
        const fetchUserList = async () => {
            if (!selectedGuild) return;
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/users?guild_id=${selectedGuild.id}`, {
                    method: 'GET',
                    headers,
                });

                if (response.ok) {
                    const data = await response.json();
                    setUsers(data);
                } else {
                    console.error('Failed to fetch user data');
                }
            } catch (error) {
                console.error('Error fetching user data:', error);
            }
        };

        fetchUserList();
    }, [selectedGuild]);

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
                    guild_id: selectedGuild?.id,
                }),
            });

            if (response.ok) {
                setLoading(false);
                setError(false);
                setDone(true);
            } else {
                setLoading(false);
                setError(true);
                console.error('Failed to pull users');
            }
        } catch (error) {
            setLoading(false);
            setError(true);
            console.error('Error pulling users:', error);
        } finally {
            setTimeout(() => {
                setResultMessage('');
                setLoading(false);
                setError(false);
                setDone(false);
            }, 5000);
        }
    };

    return (
        <div className="p-4 mt-5">
            <div className="flex items-center mb-8">
                <div className="mr-4">
                    <GuildSelect onGuildSelect={setSelectedGuild}/>
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
