import React from 'react';

interface SidebarNavItem {
    title: string;
    href: string;
    customRender?: () => React.ReactNode;
}

interface SidebarNavProps {
    items: SidebarNavItem[];
}

export const SidebarNav: React.FC<SidebarNavProps> = ({ items }) => {
    return (
        <nav className="space-y-2">
            {items.map((item, index) => (
                <div key={index} >
                    {item.customRender ? (
                        item.customRender()
                    ) : (
                        <a className="block">
                            <button className="py-2 px-4 rounded-md bg-gray-800 hover:bg-gray-700 focus:bg-gray-700">
                                {item.title}
                            </button>
                        </a>
                    )}
                </div>
            ))}
        </nav>
    );
};
