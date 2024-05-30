import React from 'react';
import { FaGithub } from 'react-icons/fa';

const Page: React.FC = () => {
    return (
        <div className="flex justify-center lg:max-w-2xl">
            <div className="bg-gray-300 p-8 rounded-lg shadow-md inline-block">
                <div className="flex items-center mb-4">
                    <FaGithub className="text-4xl mr-2 text-gray-600" />
                    <h2 className="text-2xl font-bold text-gray-800">Checkout GitHub</h2>
                </div>
                <p className="text-gray-700 mb-4">
                    Welcome! Here you can find the source code for this project on GitHub. If you encounter any issues or have suggestions for improvements, please feel free to create an issue on GitHub.
                </p>
                <a href="https://github.com/arteiii/membrs" target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">Go to GitHub</a>
                <p className="text-gray-500 text-sm mt-2">For more information, check out our <a href="https://github.com/Arteiii/membrs/wiki" className="text-blue-500 hover:underline">documentation</a>.</p>
            </div>
        </div>
    );
};

export default Page;
