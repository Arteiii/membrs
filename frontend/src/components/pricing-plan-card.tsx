import React from 'react';
import CheckedBox from '@public/assets/logos/CheckedBox.svg'
import Image from 'next/image'

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
                <Image
                    src={CheckedBox}
                    className="mr-1"
                    loading="lazy"
                    alt="Caret Icon"
                />
                <p>{benefit}</p>
            </div>
        ))}

        <button className="border-2 border-solid border-black rounded-xl text-lg py-3 mt-4">
            Choose plan
        </button>
    </div>
);

export default PricingPlanCard;
