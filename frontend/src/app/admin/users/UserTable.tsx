'use client';

import Image from 'next/image';
import { FC } from 'react';

// Define the user type
interface User {
    id: string;
    avatar: string | null;
    username: string;
    email: string;
    discord_id: string;
    expires_at: string;
}

// Define the props type
interface UserTableProps {
    users: User[];
}

const UserTable: FC<UserTableProps> = ({ users }) => {
    const imageLoader = ({ src, width }: { src: string; width: number }) => {
        return `https://cdn.discordapp.com/avatars/${src}?size=${width}`;
    };

    return (
        <div className="overflow-x-auto md:w-auto md:max-w-full md:mx-auto">
            <table className="w-full md:w-max border-collapse rounded-lg overflow-hidden bg-gray-800 shadow-md">
                <thead>
                <tr>
                    <th className="px-4 py-2 border-r border-gray-700 bg-gray-700 text-left">Avatar</th>
                    <th className="px-4 py-2 border-r border-gray-700 bg-gray-700 text-left">Username</th>
                    <th className="px-4 py-2 border-r border-gray-700 bg-gray-700 text-left">Email</th>
                    <th className="px-4 py-2 border-r border-gray-700 bg-gray-700 text-left">Discord ID</th>
                    <th className="px-4 py-2 bg-gray-700 text-left">Expires At</th>
                </tr>
                </thead>
                <tbody>
                {users.map((user, index) => (
                    <tr key={user.id} className={index % 2 === 0 ? 'bg-gray-900' : 'bg-gray-800'}>
                        <td className="px-4 py-2 border-r border-gray-700">
                            {user.avatar ? (
                                <div className="w-12 h-12 overflow-hidden rounded-full border-2 border-gray-700">
                                    <Image
                                        loader={imageLoader}
                                        src={`${user.discord_id}/${user.avatar}`}
                                        alt={`Avatar of ${user.username}`}
                                        width={48}
                                        height={48}
                                        loading="lazy"
                                    />
                                </div>
                            ) : (
                                <div className="flex items-center justify-center w-12 h-12 bg-gray-700 rounded-full">No Avatar</div>
                            )}
                        </td>
                        <td className="px-4 py-2 border-r border-gray-700">{user.username}</td>
                        <td className="px-4 py-2 border-r border-gray-700">{user.email}</td>
                        <td className="px-4 py-2 border-r border-gray-700">{user.discord_id}</td>
                        <td className="px-4 py-2">{user.expires_at}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
};

export default UserTable;
