interface Props {
    redirectUrl: string;
}

const RedirectPage: React.FC<Props> = ({ redirectUrl }) => {
    // Redirect immediately on the client-side after the page is loaded
    if (typeof window !== 'undefined') {
        window.location.href = redirectUrl;
    }

    return (
    <>
    </>
    );
};

export default RedirectPage;
