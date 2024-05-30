'use client'

import React, {useEffect, useState} from 'react';

interface ButtonProps {
    loadingClass?: string;
    errorClass?: string;
    doneClass?: string;
    disabledClass?: string;
    loading?: boolean;
    error?: boolean;
    done?: boolean;
    disabled?: boolean;
    children: React.ReactNode; // Adjusted prop type
    onClick: () => void;
}

const Button: React.FC<ButtonProps> = ({
                                           loadingClass = '',
                                           errorClass = '',
                                           doneClass = '',
                                           disabledClass = '',
                                           loading = false,
                                           error = false,
                                           done = false,
                                           disabled = false,
                                           children,
                                           onClick,
                                       }) => {
    const [isClient, setIsClient] = useState(false);
    const [iconClass, setIconClass] = useState('absolute right-2 top-1/2 transform -translate-y-1/2');

    useEffect(() => {
        setIsClient(true);
    }, []);

    return (
        <button
            className={`transition-all overflow-hidden text-white px-4 py-3 rounded-lg shadow-lg ${loading ? loadingClass : ''} ${disabled ? disabledClass : ''} ${error ? errorClass : ''} ${done ? doneClass : ''} ${loading || error || done ? 'pr-8 pl-4' : 'rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600'}`}
            disabled={disabled}
            onClick={onClick}
        >
            {children}

            {loading && (
                <span className={iconClass}>
          <svg className="spinner" viewBox="0 0 50 50">
            <circle className="path" cx="25" cy="25" r="20" fill="none" strokeWidth="5"/>
          </svg>
        </span>
            )}
            {error && (
                <span className={iconClass}>
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 fill-current" viewBox="0 0 20 20"
               fill="currentColor">
            <path fillRule="evenodd"
                  d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
                  clipRule="evenodd"/>
          </svg>
        </span>
            )}
            {done && (
                <span className={iconClass}>
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fillRule="evenodd"
                  d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                  clipRule="evenodd"/>
          </svg>
        </span>
            )}
        </button>
    );
};

export default Button;
