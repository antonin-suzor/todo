export function GET({ cookies, params }) {
    let id = parseInt(params.id)
    if (!Number.isNaN(id) && id > 0) {
        cookies.set('userId', params.id, {path: '/'})
    }
    else {
        cookies.delete('userId', {path: '/'});
    }
    return new Response();
}