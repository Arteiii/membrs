import React from 'react';

// Define an array of steps for the process
const steps = [
    {
        title: 'Login with Discord',
        description: 'Use your Discord account to log in.'
    },
    {
        title: 'Join the Server',
        description: 'Safely join the Discord server and enjoy.'
    },
    {
        title: 'Rejoin if Needed',
        description: 'Rejoin a new server if the old one got deleted.'
    }
];

export default function HowItWorks() {
    return (
        <section id="HowItWorks" className="bg-black text-white sectionSize">
            <div>
                <h2 className="secondaryTitle bg-underline2 bg-100%">How it works</h2>
            </div>
            <div className="flex flex-col md:flex-row">
                {steps.map((step, index) => (
                    <div key={index} className="flex-1 mx-8 flex flex-col items-center my-4">
                        <div className="border-2 rounded-full bg-secondary text-black h-12 w-12 flex justify-center items-center mb-3">
                            {index + 1}
                        </div>
                        <h3 className="font-montserrat font-medium text-xl mb-2">{step.title}</h3>
                        <p className="text-center font-montserrat">
                            {step.description}
                        </p>
                    </div>
                ))}
            </div>
        </section>
    );
}
