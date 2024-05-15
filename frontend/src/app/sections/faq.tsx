"use client";

import { useEffect, useState } from 'react';

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
        question: "Where can I get this HTML template?",
        answer: "You can download it on Gumroad.com"
    },
    {
        question: "Is this HTML template free?",
        answer: "Yes! For you it is free."
    },
    {
        question: "Am I awesome?",
        answer: "Yes! No doubt about it."
    }
];

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
        <section id="faq" className="sectionSize items-start pt-8 md:pt-36 bg-black text-white">
            <div>
                <h2 className="secondaryTitle bg-highlight3 p-10 mb-0 bg-center bg-100%">
                    FAQ
                </h2>
            </div>

            {faqData.map((faq, index) => (
                <div key={String(index)} className="w-full py-4" style={{ cursor: 'pointer' }} onClick={() => toggleAnswer(index)}>
                    <div className="flex justify-between items-center">
                        <div className="font-montserrat font-medium mr-auto">
                            {faq.question}
                        </div>
                        <img
                            src='./assets/logos/CaretRight.svg'
                            alt=""
                            className={`transform transition-transform ${answersState[index] ? 'rotate-90' : ''}`}
                        />
                    </div>
                    <div className={`font-montserrat text-sm font-extralight pb-8 ${answersState[index] ? '' : 'hidden'}`}>
                        {faq.answer}
                    </div>
                    <hr className="w-full bg-white" />
                </div>
            ))}
        </section>
    );
}
