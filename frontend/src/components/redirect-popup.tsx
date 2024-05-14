"use-client";

import React, { useState, useEffect } from 'react';
import axios from 'axios';

interface Props {
    redirectUrl: string;
}

const RedirectPopup: React.FC<Props> = ({ redirectUrl }) => {
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        if (!redirectUrl) return;

        const fetchData = async () => {
            try {
                // Fetch redirect URL from API
                const response = await axios.get(redirectUrl);
                console.log(response.data); // Log the received data

                setIsLoading(false);

                // Perform the actual redirect after some delay to show the animation
                window.location.href = response.data;
            } catch (error) {
                console.error('Error fetching redirect URL:', error);
            }
        };

        fetchData();
    }, [redirectUrl]);

    return (
        <div className="fixed inset-0 flex justify-center items-center bg-gray-800 bg-opacity-50">
            {isLoading ? (
                <div className="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
            ) : null}
        </div>
    );
};

export default RedirectPopup;
