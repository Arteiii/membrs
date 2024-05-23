import { GitHubLogoIcon } from "@radix-ui/react-icons";

export default function Footer() {
    return (
        <footer id="footer" className="bg-black sectionSize">
            <div className="mb-4">
                <a href="https://github.com/Arteiii/membrs">
                    <GitHubLogoIcon width={35} height={35} className="text-white"></GitHubLogoIcon>
                </a>
            </div>
        </footer>
    );
}
