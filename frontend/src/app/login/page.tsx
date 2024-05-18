import RedirectPopup from '@/components/redirect-popup';

const Page: React.FC = () => {
    const redirectUrl = `${process.env.NEXT_PUBLIC_BACKEND_URL}/oauth/url`;

    return (
        <div className="bg-gray-800 p-4">
            <RedirectPopup redirectUrl={redirectUrl}/>
        </div>
    );
};

export default Page;
