import React from 'react';
import { useEffect, useState } from 'react';
import Image from 'next/image'
import CaretRight from "@public/assets/logos/CaretRight.svg"

// Custom hook to handle client-side rendering
function useClient() {
    const [isClient, setIsClient] = useState(false);

    useEffect(() => {
        setIsClient(true);
    }, []);

    return isClient;
}

interface FaqItem {
    question: string;
    answer: string;
}

// Define FAQ data
const faqData: FaqItem[] = [
    {
        question: "Who can add me to a server?",
        answer: "Only the owner of this application."
    },
    {
        question: "Is it safe to authenticate with Discord?",
        answer: "Yes, it's safe. Discord OAuth works just like other logins. We only need permissions to join Discord servers."
    },
    {
        question: "How can I opt out of this?",
        answer: "To opt out, simply remove the app from your Discord settings. All stored data will be deleted when you do."
    },
    {
        question: "What data do you store?",
        answer: "We don't store any unusual data. It's just what's necessary for OAuth apps."
    },
    {
        question: "How do I remove the app from my Discord account?",
        answer: "To remove the app:<br/>  1. Go to your Discord settings.<br/>  2. Open 'Authorized Apps'.<br/>  3. Select the app.<br/>  4. Click 'Deauthorize'."
    }
]

// Define toggledAnswers as an array of booleans
const toggledAnswers: boolean[] = new Array(faqData.length).fill(false);

export default function Faq() {
    // Use the custom hook to determine if the component is being rendered on the client
    const isClient = useClient();

    // State to track which FAQ answers are toggled
    const [answersState, setAnswersState] = useState<boolean[]>(toggledAnswers);

    // Function to toggle an answer
    const toggleAnswer = (index: number) => {
        setAnswersState(prevState => {
            const newState = [...prevState];
            newState[index] = !newState[index];
            return newState;
        });
    };

    return (
        <section id="FAQ" className="sectionSize items-start pt-8 md:pt-36 bg-black text-white">
            <div>
                <h2 className="secondaryTitle bg-highlight3 p-10 mb-0 bg-center bg-100%">
                    FAQ
                </h2>
            </div>

            {faqData.map((faq, index) => (
                <div key={String(index)} className="w-full py-4" style={{ cursor: 'pointer' }}
                     onClick={() => toggleAnswer(index)}>
                    <div className="flex justify-between items-center">
                        <div className="font-montserrat font-medium mr-auto">
                            {faq.question}
                        </div>
                        <Image
                            src={CaretRight}
                            className={`transform transition-transform ${answersState[index] ? 'rotate-90' : ''}`}
                            loading="lazy"
                            alt="Caret Icon"
                        />
                    </div>
                    <div
                        className={`font-montserrat text-sm font-extralight pb-8 ${answersState[index] ? '' : 'hidden'}`}
                        dangerouslySetInnerHTML={{ __html: faq.answer }} // Render HTML markup
                    ></div>
                    <hr className="w-full bg-white" />
                </div>
            ))}
        </section>
    );
}
