'use client';
import React, { useEffect, useState } from 'react';
import Button from '@/components/button';
import ChangeUserData from '../components/update_user';
import GuildSelect from '../components/guild-select';

const Page: React.FC = () => {
    const [configData, setConfigData] = useState<any>({});
    const [editableFields, setEditableFields] = useState<{ [key: string]: boolean }>({});
    const [resultMessage, setResultMessage] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(false);
    const [done, setDone] = useState(false);
    const [showChangeUserData, setShowChangeUserData] = useState(false);

    useEffect(() => {
        const fetchConfigData = async () => {
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/config`, {
                    headers,
                });

                if (response.ok) {
                    const data = await response.json();
                    setConfigData(data);
                } else if (response.status === 401) {
                    console.error('Invalid username or password');
                    localStorage.removeItem('username');
                    localStorage.removeItem('password');
                } else {
                    console.error('Failed to fetch config data');
                }
            } catch (error) {
                console.error('Error fetching config data:', error);
            }
        };

        fetchConfigData();
    }, []);

    const handleFieldChange = (field: string, value: string) => {
        setConfigData((prevConfigData: any) => ({
            ...prevConfigData,
            [field]: value,
        }));
    };

    const toggleEdit = (field: string) => {
        setEditableFields((prevEditableFields) => ({
            ...prevEditableFields,
            [field]: !prevEditableFields[field],
        }));
    };

    const handleSave = async () => {
        setLoading(true);
        setError(false);
        setDone(false);
        try {
            const username = localStorage.getItem('username');
            const password = localStorage.getItem('password');
            const headers = new Headers();
            headers.set('Content-Type', 'application/json');
            if (username && password) {
                headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
            }

            const response = await fetch(`${process.env.NEXT_PUBLIC_URL}/api/superuser/config`, {
                method: 'POST',
                headers,
                body: JSON.stringify(configData),
            });

            if (response.ok) {
                setLoading(false);
                setDone(true);
                setResultMessage('Config data saved successfully!');
            } else {
                setLoading(false);
                setError(true);
                setResultMessage('Failed to save config data');
            }
        } catch (error) {
            console.error('Error saving config data:', error);
            setLoading(false);
            setError(true);
            setResultMessage('Error saving config data');
        } finally {
            setLoading(false);
            setTimeout(() => {
                setResultMessage('');
                setLoading(false);
                setError(false);
                setDone(false);
            }, 5000);
        }
    };

    const handleGuildSelect = (guild: { id: string; name: string; icon: string } | null) => {
        if (guild) {
            handleFieldChange('guild_id', guild.id);
        }
    };

    const renderEditableField = (label: string, field: string) => (
        <div className="mb-4 flex items-center">
            <div className="font-bold mr-2">{label}:</div>
            {editableFields[field] ? (
                <input
                    type="text"
                    value={configData[field] || ''}
                    onChange={(e) => handleFieldChange(field, e.target.value)}
                    className="bg-gray-700 text-white rounded p-1 flex-grow"
                />
            ) : (
                <div>{configData[field] || ''}</div>
            )}
            <button onClick={() => toggleEdit(field)} className="ml-2">
                ✏️
            </button>
        </div>
    );

    return (
        <div className="p-4 mt-5">
            <div className="flex justify-between items-center mb-4">
                <Button onClick={() => setShowChangeUserData(true)}>
                    Change User Data
                </Button>
                {showChangeUserData && <ChangeUserData onChangeSuccess={() => setShowChangeUserData(false)} />}
            </div>

            <div className="lg:max-w-2xl bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                <div className="text-2xl font-bold mb-4">Client Information</div>
                {renderEditableField('Client ID', 'client_id')}
                {renderEditableField('Client Secret', 'client_secret')}
                {renderEditableField('Redirect URI', 'redirect_uri')}
            </div>

            <div className="lg:max-w-2xl bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                <div className="text-2xl font-bold mb-4">Bot</div>
                {renderEditableField('Bot Token', 'bot_token')}
                <div className="mb-4 flex items-center">
                    <div className="font-bold mr-2">Guild ID:</div>
                    <div className="mr-4">
                        <GuildSelect onGuildSelect={handleGuildSelect}/>
                    </div>
                </div>
                <div className="mb-4">
                    <div className="font-bold">OAuth URL:</div>
                    <a href={configData.oauth_url} className="text-blue-500 underline">
                        {configData.oauth_url || ''}
                    </a>
                </div>
            </div>

            <div className="flex justify-start mb-4">
                <Button
                    loading={loading}
                    loadingClass="bg-yellow-600 scale-110 active:bg-yellow-600"
                    error={error}
                    errorClass="bg-red-600 scale-110 shake active:bg-red-600"
                    done={done}
                    doneClass="bg-green-600 scale-110 active:bg-green-600"
                    onClick={handleSave}
                >
                    {resultMessage ? resultMessage : 'Save'}
                </Button>
            </div>
        </div>
    );
};

export default Page;
