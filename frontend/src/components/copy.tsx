import { useState } from 'react';
import { FaCopy } from 'react-icons/fa'; // Import the copy icon

interface CopyToClipboardProps {
    text: string;
}

const CopyToClipboard: React.FC<CopyToClipboardProps> = ({ text }) => {
    const [copied, setCopied] = useState<boolean>(false);

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(text);
            setCopied(true);
            // Reset copied state after a short delay
            setTimeout(() => setCopied(false), 2000);
        } catch (error) {
            console.error('Failed to copy text: ', error);
        }
    };

    return (
        <div>
            <button onClick={handleCopy} className={`flex items-center ${copied ? 'text-green-500' : ''}`}>
                <FaCopy className="mr-2" /> {/* Copy icon */}
                {copied ? 'Copied!' : 'Copy'}
            </button>
        </div>
    );
};

export default CopyToClipboard;
