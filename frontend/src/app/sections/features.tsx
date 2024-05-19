import React from 'react';
import Image from 'next/image'

// Define an array of feature items
const featureItems = [
    {
        title: 'Feature #1',
        description: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quisquam voluptate praesentium tenetur earum repellendus! Dicta culpa consequuntur saepe quibusdam labore, est ex ducimus tempore, quos illum officiis, pariatur ea placeat.',
        image: './assets/logos/Heart.svg'
    },
    {
        title: 'Feature #2',
        description: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quisquam voluptate praesentium tenetur earum repellendus! Dicta culpa consequuntur saepe quibusdam labore, est ex ducimus tempore, quos illum officiis, pariatur ea placeat.',
        image: './assets/logos/Heart.svg'
    },
    {
        title: 'Feature #3',
        description: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quisquam voluptate praesentium tenetur earum repellendus! Dicta culpa consequuntur saepe quibusdam labore, est ex ducimus tempore, quos illum officiis, pariatur ea placeat.',
        image: './assets/logos/Heart.svg'
    },
    {
        title: 'Feature #88',
        description: 'Lorem ipsum dolor sit amet consectetur, adipisicing elit. Quisquam voluptate praesentium tenetur earum repellendus! Dicta culpa consequuntur saepe quibusdam labore, est ex ducimus tempore, quos illum officiis, pariatur ea placeat.',
        image: './assets/logos/Heart.svg'
    }
];

export default function Features() {
    return (
        <section id="Features" className="sectionSize bg-secondary">
            <div>
                <h2 className="secondaryTitle bg-underline3 bg-100%">Features</h2>
            </div>
            <div className="md:grid md:grid-cols-2 md:grid-rows-2">
                {featureItems.map((item, index) => (
                    <div key={index} className="flex items-start font-montserrat my-6 mr-10">
                        <Image
                            src={item.image}
                            alt={`${item.title} icon`}
                            width={28}
                            height={28}
                            className="mr-4"
                            loading="lazy"
                        />
                        <div>
                            <h3 className="font-semibold text-2xl">{item.title}</h3>
                            <p>{item.description}</p>
                        </div>
                    </div>
                ))}
            </div>
        </section>
    );
}
