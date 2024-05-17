import React from 'react';

const AdminLogin: React.FC = () => {
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
                        <form className="mt-8 space-y-6" action="#" method="POST">
                            <div className="rounded-md shadow-sm -space-y-px">
                                <div>
                                    <label htmlFor="email-address" className="sr-only">
                                        Email address
                                    </label>
                                    <input
                                        id="username"
                                        name="username"
                                        type="username"
                                        autoComplete="username"
                                        required
                                        className="appearance-none bg-slate-800 rounded-none relative block w-full px-3 py-2 border border-gray-700 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="Username"
                                    />
                                </div>
                                <div>
                                    <label htmlFor="password" className="sr-only">
                                        Password
                                    </label>
                                    <input
                                        id="password"
                                        name="password"
                                        type="password"
                                        autoComplete="current-password"
                                        required
                                        className="appearance-none rounded-none bg-slate-800 relative block w-full px-3 py-2 border border-gray-700 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                        placeholder="Password"
                                    />
                                </div>
                            </div>

                            <div>
                                <button
                                    type="submit"
                                    className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                >
                                    Sign in
                                </button>
                            </div>
                        </form>
                        <p className="mt-2 text-center text-sm text-gray-600">
                            Not the owner?{' '}
                            <a
                                href="https://github.com/Arteiii/membrs/"
                                className="font-medium text-indigo-600 hover:text-indigo-500"
                            >
                                Host your own version
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AdminLogin;
