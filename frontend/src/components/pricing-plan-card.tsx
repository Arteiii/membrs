import React from 'react';

interface PricingPlanCardProps {
    title: string;
    price: string;
    period: string;
    benefits: string[];
    topOffset: string;
}

const PricingPlanCard: React.FC<PricingPlanCardProps> = ({ title, price, period, benefits, topOffset }) => (
    <div className={`flex-1 flex flex-col mx-6 shadow-2xl relative bg-secondary rounded-2xl py-5 px-8 my-8 ${topOffset}`}>
        <h3 className="font-pt-serif font-normal text-2xl mb-4">
            {title}
        </h3>
        <div className="font-montserrat font-bold text-2xl mb-4">
            {price}
            <span className="font-normal text-base"> {period}</span>
        </div>

        {benefits.map((benefit, index) => (
            <div className="flex" key={index}>
                <img src='./assets/logos/CheckedBox.svg' alt="" className="mr-1" />
                <p>{benefit}</p>
            </div>
        ))}

        <button className="border-2 border-solid border-black rounded-xl text-lg py-3 mt-4">
            Choose plan
        </button>
    </div>
);

export default PricingPlanCard;
