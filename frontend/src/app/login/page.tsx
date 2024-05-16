import RedirectPopup from '@/components/redirect-popup';

const Page: React.FC = () => {
    const baseUrl  = process.env.BACKEND_URL;

    const redirectUrl = `${baseUrl}/oauth/url`;

    return (
        <div className="bg-gray-800 p-4">
            <RedirectPopup redirectUrl={redirectUrl}/>
        </div>
    );
};

export default Page;
