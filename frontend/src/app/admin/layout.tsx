'use client'

import React, {useEffect, useState} from 'react';
import {Separator} from '@/components/ui/separator';
import {SidebarNav} from './components/sidebar-nav';
import AdminLogin from './components/login';
import Link from 'next/link';
import {FaCog, FaUsers, FaArrowRight, FaArrowLeft} from 'react-icons/fa'; // Importing icons from react-icons

const sidebarNavItems = [
    {
        title: 'Settings',
        href: '/admin/settings',
        icon: <FaCog size={28}/>
    },
    {
        title: 'Users',
        href: '/admin/users',
        icon: <FaUsers size={28}/>
    },
];

interface SettingsLayoutProps {
    children: React.ReactNode;
}

const SettingsLayout: React.FC<SettingsLayoutProps> = ({children}) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(true);
    const [isExpanded, setIsExpanded] = useState<boolean>(false);
    const [showText, setShowText] = useState<boolean>(false);

    useEffect(() => {
        const username = localStorage.getItem('username');
        const password = localStorage.getItem('password');

        if (!username || !password) {
            setIsAuthenticated(true);
        }
    }, []);

    useEffect(() => {
        if (isExpanded) {
            const timeoutId = setTimeout(() => {
                setShowText(true);
            }, 300); // Duration of the sidebar transition
            return () => clearTimeout(timeoutId);
        } else {
            setShowText(false);
        }
    }, [isExpanded]);

    const handleLoginSuccess = () => {
        setIsAuthenticated(true);
    };

    const toggleSidebar = () => {
        setIsExpanded(!isExpanded);
    };

    return (
        <div className="min-h-screen bg-gray-900 text-gray-300">

            <div className="absolute inset-0 flex justify-center items-center">
                {!isAuthenticated && <AdminLogin onLoginSuccess={handleLoginSuccess}/>}
            </div>

            <div className={`${isAuthenticated ? '' : 'blur-sm'}`}>
                <div
                    className={`sidebar-overlay mb-5 absolute top-0 left-0 h-full bg-gray-800 p-4 ${isExpanded ? 'w-64' : 'w-16'} transition-all duration-300 overflow-hidden flex flex-col justify-between items-center`}
                >
                    <Link href="/admin/" className="text-center">
                        <div className="mb-3">
                            <h1 className={`text-3xl font-bold bg-gradient-to-r from-purple-500 via-red-500 to-yellow-500 text-transparent bg-clip-text transition-transform duration-500 transform delay-200`}>
                                {isExpanded ? 'Superuser' : 'S'}
                            </h1>
                        </div>
                        <p className={`text-gray-400 transition-opacity duration-500 ${showText ? 'opacity-100' : 'opacity-0'}`}>
                            Manage your authenticated members
                        </p>
                    </Link>
                    <SidebarNav
                        items={sidebarNavItems.map((item) => ({
                            ...item,
                            customRender: () => (
                                <a href={item.href} className="flex items-center space-x-4">
                                    <div className="text-gray-400">
                                        {item.icon}
                                    </div>
                                    <span
                                        className={`text-gray-300 transition-opacity duration-300`}
                                    >
                                        {showText ? item.title : ''}
                                    </span>
                                </a>
                            ),
                        }))}
                    />

                    <button
                        onClick={toggleSidebar}
                        className="text-gray-300 flex items-center justify-center p-3 rounded bg-gray-700 hover:bg-gray-600 focus:bg-gray-600 w-full"
                    >
                        {isExpanded ? <FaArrowLeft/> : <FaArrowRight/>}
                        <span className={`ml-2 ${isExpanded ? 'block' : 'hidden'}`}>Collapse</span>
                    </button>
                </div>

                <div className={`${isExpanded ? 'ml-72' : 'ml-24'} transition-all duration-300 absolute h-full`}>
                    {children}
                </div>
            </div>
        </div>
    );
};

export default SettingsLayout;
