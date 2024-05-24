'use client';
import React, { useEffect, useState } from 'react';

const Page: React.FC = () => {
    const [configData, setConfigData] = useState<any>(null);
    const [editableFields, setEditableFields] = useState<{ [key: string]: boolean }>({});

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
                console.log('Config data saved successfully');
            } else {
                console.error('Failed to save config data');
            }
        } catch (error) {
            console.error('Error saving config data:', error);
        }
    };

    const renderEditableField = (label: string, field: string) => (
        <div className="mb-4 flex items-center">
            <div className="font-bold mr-2">{label}:</div>
            {editableFields[field] ? (
                <input
                    type="text"
                    value={configData[field]}
                    onChange={(e) => handleFieldChange(field, e.target.value)}
                    className="bg-gray-700 text-white rounded p-1 flex-grow"
                />
            ) : (
                <div>{configData[field]}</div>
            )}
            <button onClick={() => toggleEdit(field)} className="ml-2">
                ✏️
            </button>
        </div>
    );

    return (
        <>
            {/* Client Information Section */}
            {configData && (
                <div className="lg:max-w-2xl bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                    <div className="text-2xl font-bold mb-4">Client Information</div>
                    {renderEditableField('Client ID', 'client_id')}
                    {renderEditableField('Client Secret', 'client_secret')}
                    {renderEditableField('Redirect URI', 'redirect_uri')}
                </div>
            )}

            {/* Other Categories */}
            {configData && (
                <div className="bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                    <div className="text-2xl font-bold mb-4">Bot</div>
                    {renderEditableField('Bot Token', 'bot_token')}
                    {renderEditableField('Guild ID', 'guild_id')}
                    <div className="mb-4">
                        <div className="font-bold">OAuth URL:</div>
                        <a href={configData.oauth_url} className="text-blue-500 underline">{configData.oauth_url}</a>
                    </div>
                </div>
            )}

            {/* Save Button */}
            <div className="flex justify-end">
                <button
                    onClick={handleSave}
                    className="bg-blue-500 text-white p-2 rounded"
                >
                    Save
                </button>
            </div>

            <style jsx>{`
                .blurred-text {
                    filter: blur(3px);
                }

                .group:hover .blur-hidden {
                    filter: none;
                }
            `}</style>
        </>
    );
};

export default Page;
