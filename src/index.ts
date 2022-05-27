import { Contributors } from './contributors';

const contributor = Contributors[Math.floor(Math.random() * Contributors.length)];

(function () {
    function load(style: string) {

        const script = document.createElement("script");
        script.type = 'text/javascript';
        script.src = `dist/scripts/${style}.js`;
        script.async = false;

        const link = document.createElement("link");
        link.type = "text/css";
        link.rel = "stylesheet";
        link.href = `dist/styles/${style}.css`

        document.getElementsByTagName("head")[0].appendChild(script);
        document.getElementsByTagName("head")[0].appendChild(link);

        console.log(`Loaded style ${style}`);
    }

    const style: string = contributor.styles[Math.floor(Math.random() * contributor.styles.length)];

    load(`${contributor.personalia.github}/${style}`);
})();
