import React, { useState } from 'react';

interface ChangeUserDataProps {
    onChangeSuccess: () => void;
}

const ChangeUserData: React.FC<ChangeUserDataProps> = ({ onChangeSuccess }) => {
    const [newUsername, setNewUsername] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [error, setError] = useState<string>('');

    const handleChange = async (e: React.FormEvent) => {
        e.preventDefault();
        const username = localStorage.getItem('username');
        const password = localStorage.getItem('password');

        const requestUrl = `${process.env.NEXT_PUBLIC_URL}/api/superuser`;

        try {
            const response = await fetch(requestUrl, {
                method: 'PUT',
                headers: {
                    'Authorization': `Basic ${btoa(`${username}:${password}`)}`,
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    new_username: newUsername,
                    new_password: newPassword,
                }),
            });

            if (response.ok) {
                console.log('Change successful');
                onChangeSuccess();
            } else if (response.status === 401) {
                setError('Invalid username or password');
                console.error('Change failed');
            } else {
                setError('Error during change. Please try again.');
                console.error('Change failed with status:', response.status);
            }
        } catch (error) {
            console.error('Error during change:', error);
            setError('Error during change. Please try again.');
        }
    };

    return (
        <div className="fixed inset-0 flex justify-center items-center backdrop-blur-md">
            <div className="absolute inset-0 bg-gray-900 bg-opacity-80"></div>
            <div className="z-10 bg-gray-800 bg-opacity-90 p-8 rounded-lg max-w-md">
                <div className="text-center">
                    <h1 className="text-5xl font-bold mb-5 bg-gradient-to-r from-purple-500 via-red-500 to-yellow-500 text-transparent bg-clip-text md:-bottom-32">
                        membrs
                    </h1>
                    <p className="text-lg text-gray-400 mb-4">
                        This is the management page for Superusers
                    </p>
                    <div className="lg:max-w-sm mx-auto">
                        <form className="mt-8 space-y-6" onSubmit={handleChange}>
                            <div className="rounded shadow-sm -space-y-px">
                                <div>
                                    <label htmlFor="newUsername" className="sr-only">
                                        New Username
                                    </label>
                                    <input
                                        id="newUsername"
                                        name="newUsername"
                                        type="text"
                                        autoComplete="new-username"
                                        required
                                        value={newUsername}
                                        onChange={(e) => setNewUsername(e.target.value)}
                                        className="appearance-none bg-slate-800 rounded-t relative block w-full px-3 py-2 border border-gray-700 placeholder-gray-500 text-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="New Username"
                                    />
                                </div>
                                <div>
                                    <label htmlFor="newPassword" className="sr-only">
                                        New Password
                                    </label>
                                    <input
                                        id="newPassword"
                                        name="newPassword"
                                        type="password"
                                        autoComplete="new-password"
                                        required
                                        value={newPassword}
                                        onChange={(e) => setNewPassword(e.target.value)}
                                        className="appearance-none rounded-b bg-slate-800 relative block w-full px-3 py-2 border border-gray-700 placeholder-gray-500 text-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="New Password"
                                    />
                                </div>
                            </div>

                            {error && <p className="text-red-500 text-sm">{error}</p>}

                            <div>
                                <button
                                    type="submit"
                                    className="group rounded relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                >
                                    Change
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default ChangeUserData;
