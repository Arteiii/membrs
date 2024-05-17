'use client';

import React, {useEffect, useState} from 'react';
import AdminLogin from "@/app/admin/components/login";

const Page: React.FC = () => {
    const [isLoggedIn, setIsLoggedIn] = useState<boolean>(false);

    useEffect(() => {
        // Check if there is a cookie stored with the JWT token
        const jwtToken = getCookie('jwtToken');
        if (jwtToken) {
            setIsLoggedIn(true);
        } else {
            setIsLoggedIn(false);
        }
    }, []);

    // Function to get cookie value by name
    const getCookie = (name: string) => {
        const cookies = document.cookie.split(';');
        for (let i = 0; i < cookies.length; i++) {
            const cookie = cookies[i].trim();
            if (cookie.startsWith(name + '=')) {
                return cookie.substring(name.length + 1);
            }
        }
        return '';
    };

    return (
        <>
            {isLoggedIn ? (
                // If logged in, display the admin content
                <div>Your admin content here</div>
            ) : (
                // If not logged in, display the admin login component
                <AdminLogin/>
            )}
        </>
    );
};

export default Page;
