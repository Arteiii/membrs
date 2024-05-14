"use client";

import RedirectPopup from '@/components/redirect-popup';
import { useEffect, useState } from 'react';


const Page: React.FC = () => {
    const [showRedirectPopup, setShowRedirectPopup] = useState(false);
    const redirectUrl = 'http://localhost:3333/oauth/url'; // Example redirect URL

    console.log(redirectUrl);

    const handleRedirect = () => {
        setShowRedirectPopup(true);
    };

    return (
        <div className="bg-gray-800 p-4">
            <RedirectPopup redirectUrl={'http://localhost:3333/oauth/url'} />
        </div>
    );
};

export default Page;
