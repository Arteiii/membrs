import React from 'react';
import PricingPlanCard from '@/components/pricing-plan-card';

const pricingPlans = [
  {
    title: 'Selfhosted',
    price: 'Free',
    period: '/ month',
    benefits: ['Benefit #1', 'Benefit #2', 'Benefit #3'],
    topOffset: 'md:top-24',
  },
  {
    title: 'The Bad',
    price: 'Free',
    period: '/ month',
    benefits: ['Benefit #1', 'Benefit #2', 'Benefit #3'],
    topOffset: 'md:top-12',
  },
  {
    title: 'The Ugly',
    price: 'Free',
    period: '/ month',
    benefits: ['Benefit #1', 'Benefit #2', 'Benefit #3'],
    topOffset: 'md:top-24',
  },
];

export default function Pricing() {
  return (
      <section id="Pricing" className="sectionSize bg-secondary py-0">
        <div>
          <h2 className="secondaryTitle bg-underline4 mb-0 bg-100%">Pricing</h2>
        </div>
        <div className="flex w-full flex-col md:flex-row">
          {pricingPlans.map((plan, index) => (
              <PricingPlanCard
                  key={index}
                  title={plan.title}
                  price={plan.price}
                  period={plan.period}
                  benefits={plan.benefits}
                  topOffset={plan.topOffset}
              />
          ))}
        </div>
      </section>
  );
}
