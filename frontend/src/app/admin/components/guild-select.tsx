'use client'

import React, { useState, useEffect } from 'react';

interface Guild {
    icon: string;
    id: string;
    name: string;
}

interface GuildSelectProps {
    onGuildSelect: (guild: Guild | null) => void;
}

const GuildSelect: React.FC<GuildSelectProps> = ({ onGuildSelect }) => {
    const [guilds, setGuilds] = useState<Guild[]>([]);
    const [selectedGuild, setSelectedGuild] = useState<Guild | null>(null);

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
                    if (data.length > 0) {
                        setSelectedGuild(data[0]);
                        onGuildSelect(data[0]);
                    }
                } else if (response.status === 401) {
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
    }, [onGuildSelect]);

    const handleSelectGuild = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const selectedGuildId = event.target.value;
        const guild = guilds.find(guild => guild.id === selectedGuildId) || null;
        setSelectedGuild(guild);
        onGuildSelect(guild);
    };

    return (
        <div className="relative">
            <div className="inline-block relative">
                <div className="flex items-center">
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
    );
};

export default GuildSelect;
