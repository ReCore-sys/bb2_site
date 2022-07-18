import axios from "redaxios";

let port: number;

// Check both the current port and the default port on the /api/ping route. Set port to whichever one is available.
export async function get(path: string) {
    const currentport = axios.get(`http://${window.location.host}/api/ping/`); // with port
    const pubport = axios.get(`http://${window.location.hostname}/api/ping/`); // without port
    const [current, pub] = await Promise.all([currentport, pubport]);
    if (path[0] === "/") {
        path = path.substring(1);
    }
    if (pub.status === 200) {
        return await axios.get(`http://${window.location.hostname}/${path}`);
    } else if (current.status === 200) {
        return await axios.get(`http://${window.location.host}/${path}`);
    } else {
        throw new Error("No port available");
    }
}

export async function post(path: string, data: any) {
    const currentport = axios.get(`http://${window.location.host}/api/ping/`); // with port
    const pubport = axios.get(`http://${window.location.hostname}/api/ping/`); // without port
    const [current, pub] = await Promise.all([currentport, pubport]);
    if (path[0] === "/") {
        path = path.substring(1);
    }
    if (pub.status === 200) {
        return await axios.post(
            `http://${window.location.hostname}/${path}`,
            data
        );
    } else if (current.status === 200) {
        return await axios.post(`http://${window.location.host}/${path}`, data);
    } else {
        throw new Error("No port available");
    }
}
