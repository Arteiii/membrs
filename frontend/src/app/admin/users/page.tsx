'use client'

import React, {useState, useEffect, SetStateAction} from 'react';
import UserTable from './UserTable';
import Button from "@/components/button";
import Image from "next/image";

export default function Home() {
    const [users, setUsers] = useState([]);
    const [guilds, setGuilds] = useState<{
        icon: string;
        id: string; name: string
    }[]>([]);
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
        const fetchGuilds = async () => {
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/bot/guilds`, {
                    method: 'GET',
                    headers,
                });

                if (response.ok) {
                    const data = await response.json();
                    setGuilds(data);
                    // Set the first guild as the default selected guild
                    if (data.length > 0) {
                        setSelectedGuild(data[0].id);
                    }
                } else if (response.status === 401) {
                    // Handle authorization error
                    console.error('Invalid username or password');
                    localStorage.removeItem('username');
                    localStorage.removeItem('password');
                } else {
                    console.error('Failed to fetch guilds data');
                }
            } catch (error) {
                console.error('Error fetching guilds data:', error);
            }
        };

        fetchGuilds();
    }, []);

    useEffect(() => {
        // Fetch user list whenever selected guild changes
        const fetchUserList = async () => {
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/users?guild_id=${selectedGuild}`, {
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
                    guild_id: selectedGuild,
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
            // Reset button state after a delay
            setTimeout(() => {
                setResultMessage('');
                setLoading(false);
                setError(false);
                setDone(false);
            }, 5000);
        }
    };

    const handleSelectGuild = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const selectedGuildId = event.target.value;
        const guild = guilds.find(guild => guild.id === selectedGuildId);
        setSelectedGuild(guild || null);
    };

    const imageLoader = ({src, width}: { src: string; width: number }) => {
        return `https://cdn.discordapp.com/icons/${src}.webp?size=${width}`;
    };

    return (
        <div className="p-4 mt-5">
            <div className="flex items-center mb-8">
                <div className="mr-2">
                    <div className="relative">
                        <label htmlFor="guildSelect" className="sr-only">
                            Select Guild
                        </label>
                        <div className="inline-block relative">
                            <div className="flex items-center">
                                {selectedGuild && (
                                    <Image
                                        loader={imageLoader}
                                        src={selectedGuild.icon}
                                        width={20}
                                        height={20}
                                        alt="test image"
                                        className="rounded-full mr-2"
                                    />
                                )}
                                <span>{selectedGuild ? selectedGuild.name : 'Select Guild'}</span>
                            </div>
                            <select
                                id="guildSelect"
                                value={selectedGuild ? selectedGuild.id : ''}
                                onChange={handleSelectGuild}
                                className="appearance-none absolute top-0 left-0 opacity-0 w-full h-full cursor-pointer"
                            >
                                {guilds.map(guild => (
                                    <option key={guild.id} value={guild.id}>
                                        {guild.name}
                                    </option>
                                ))}
                            </select>
                        </div>
                    </div>
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
