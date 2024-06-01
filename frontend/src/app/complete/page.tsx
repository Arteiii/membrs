'use client'

import Head from 'next/head';
import { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import Image from "next/image";
import CaretRight from '@public/assets/logos/CaretRight.svg';
import CopyToClipboard from '@/components/copy';

function CompletePageContent() {
    const searchParams = useSearchParams();
    const [errorDetailsVisible, setErrorDetailsVisible] = useState(false);

    const toggleErrorDetails = () => {
        setErrorDetailsVisible(prevState => !prevState);
    };

    const status = searchParams.get('status');
    const username = searchParams.get('username') || '';
    const errorText = searchParams.get('error') || '';
    const profilePicture = searchParams.get('profile_picture') || '';

    const imageLoader = ({ src, width }: { src: string; width: number }) => {
        return `${src}?size=${width}`;
    };

    return (
        <div className="min-h-screen flex justify-center items-center bg-gray-100">
            <Head>
                <title>{status === 'complete' ? 'Login Complete' : 'Login Failed'}</title>
            </Head>
            <div className="max-w-md p-8 bg-white rounded-lg shadow-lg">
                {status === 'complete' ? (
                    <div>
                        <h1 className="text-3xl font-semibold mb-4">Login Complete!</h1>
                        <div className="flex items-center mb-4">
                            {profilePicture && (
                                <div className="rounded-full overflow-hidden mr-4">
                                    <Image
                                        loader={imageLoader}
                                        src={profilePicture}
                                        alt={`Avatar`}
                                        width={48}
                                        height={48}
                                        loading="lazy"
                                    />
                                </div>
                            )}
                            <p className="text-lg">Welcome, {username}!</p>
                        </div>
                    </div>
                ) : (
                    <div>
                        <h1 className="text-3xl font-semibold mb-4">Login Failed!</h1>
                        <p className="text-lg mb-4">Sorry, there was an error processing your login request.</p>
                        <div className="flex justify-between items-center cursor-pointer" onClick={toggleErrorDetails}>
                            <div className="font-montserrat font-medium mr-auto">
                                Error Details
                            </div>
                            <Image
                                src={CaretRight}
                                alt="caret right"
                                className={`transform transition-transform ${errorDetailsVisible ? 'rotate-90' : ''}`}
                                width={40}
                                height={40}
                                style={{ filter: 'invert(100%)' }}
                            />
                        </div>
                        {errorDetailsVisible && (
                            <div className="flex justify-between items-center cursor-pointer">
                                <p className="text-s mb-4 text-color" style={{ color: '#333', opacity: 0.6 }}>
                                    {errorText}
                                </p>
                                <CopyToClipboard text={errorText} />
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}

export default function CompletePage() {
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <CompletePageContent />
        </Suspense>
    );
}
