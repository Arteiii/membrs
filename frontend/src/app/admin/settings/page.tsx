'use client';
import React, {useEffect, useState} from 'react';

const Page: React.FC = () => {
    const [configData, setConfigData] = useState<any>(null);

    useEffect(() => {
        const fetchConfigData = async () => {
            try {
                const username = localStorage.getItem('username');
                const password = localStorage.getItem('password');
                const headers = new Headers();
                if (username && password) {
                    headers.set('Authorization', `Basic ${btoa(`${username}:${password}`)}`);
                }

                const response = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/superuser/config`, {
                    headers,
                });
                if (response.ok) {
                    const data = await response.json();
                    setConfigData(data);
                } else {
                    console.error('Failed to fetch config data');
                }
            } catch (error) {
                console.error('Error fetching config data:', error);
            }
        };

        fetchConfigData();
    }, []);

    return (
        <>
            {/* Client Information Section */}
            {configData && (
                <div className="lg:max-w-2xl bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                    <div className="text-2xl font-bold mb-4">Client Information</div>
                    <div className="flex">
                        <div className="mr-28">
                            <div className="font-bold">Client ID:</div>
                            <div>{configData.client_id}</div>
                        </div>
                        <div>
                            <div className="font-bold">Client Secret:</div>
                            <div>{configData.client_secret}</div>
                        </div>
                    </div>
                </div>
            )}

            {/* Other Categories */}
            {configData && (
                <div className="bg-gray-800 p-6 rounded-lg shadow-md text-white mb-4">
                    <div className="text-2xl font-bold mb-4">Other Categories</div>
                    <div className="font-bold mb-2">Backend URL:</div>
                    <div>{configData.backend_url}</div>
                    <div className="font-bold mb-2">Frontend URL:</div>
                    <div>{configData.frontend_url}</div>
                    <div className="font-bold mb-2">Bot Token:</div>
                    <div>{configData.bot_token}</div>
                    <div className="font-bold mb-2">OAuth URL:</div>
                    <div><a href={configData.oauth_url} className="text-blue-500 underline">Link</a></div>
                    <div className="font-bold mb-2">Redirect URI:</div>
                    <div>{configData.redirect_uri}</div>
                    <div className="font-bold mb-2">Guild ID:</div>
                    <div>{configData.guild_id}</div>
                </div>
            )}

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
