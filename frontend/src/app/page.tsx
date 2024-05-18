"use client"

import Navigation from "./sections/navigation";
import Footer from "./sections/footer";
import Faq from "./sections/faq";
import Pricing from "./sections/pricing";
import Features from "./sections/features";
import HowItWorks from "./sections/howItWorks";
import Hero from "./sections/hero";

export default function Home() {
  return (
      <main>
          <Navigation/>
          <Hero/>
          <HowItWorks/>
          <Features/>
          {/*<Pricing />*/}
          <Faq/>
          <Footer/>
      </main>
  );
}
