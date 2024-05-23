'use client';

import React, {useEffect, useState} from 'react';
import {Separator} from '@/components/ui/separator';
import {SidebarNav} from './components/sidebar-nav';
import AdminLogin from './components/login';

const sidebarNavItems = [
    {
        title: 'Settings',
        href: '/admin/settings',
    },
    {
        title: 'Users',
        href: '/admin/users',
    },
];

interface SettingsLayoutProps {
    children: React.ReactNode;
}

const SettingsLayout: React.FC<SettingsLayoutProps> = ({children}) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(true);

    useEffect(() => {
        const username = localStorage.getItem('username');
        const password = localStorage.getItem('password');

        if (!username || !password) {
            setIsAuthenticated(false);
        }
    }, []);

    const handleLoginSuccess = () => {
        setIsAuthenticated(true);
    };

    return (
        <>
            {!isAuthenticated && <AdminLogin onLoginSuccess={handleLoginSuccess}/>}
            <div className="min-h-screen bg-gray-900 text-gray-300">
                <div className="hidden space-y-6 p-10 pb-16 md:block">
                    <div className="space-y-0.5">
                        <h2 className="text-2xl font-bold tracking-tight">Settings</h2>
                        <p className="text-gray-400">
                            Manage your authenticated membrs
                        </p>
                    </div>
                    <Separator className="my-6 border-gray-700"/>
                    <div className="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
                        <aside className="-mx-4 lg:w-1/5">
                            <SidebarNav items={sidebarNavItems.map(item => ({
                                ...item,
                                customRender: () => (
                                    <a href={item.href}>
                                        <button
                                            className="w-48 text-left py-2 px-4 rounded-md bg-gray-800 hover:bg-gray-700 focus:bg-gray-700">
                                            {item.title}
                                        </button>
                                    </a>
                                )
                            }))}/>
                        </aside>
                        <div className="flex-1 ">{children}</div>
                    </div>
                </div>
            </div>
        </>
    );
};

export default SettingsLayout;
