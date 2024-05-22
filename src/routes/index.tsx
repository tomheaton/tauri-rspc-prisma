import { api, invalidateQuery } from "@/main";
import { useState } from "react";

export default function Index() {
  const { data: version } = api.useQuery(["version"]);
  const { data: posts } = api.useQuery(["posts"]);

  const [title, setTitle] = useState<string>("");
  const [content, setContent] = useState<string>("");

  const createPostMutation = api.useMutation(["createPost"], {
    onSuccess: () => {
      console.log("Post created!");

      // TODO: add official invalidation
      // api.query.invalidate(["posts"]);

      // NOTE: this is a workaround for invalidating the query
      invalidateQuery("posts");

      setTitle("");
      setContent("");
    },
    onError: (e) => {
      console.error(e);
    },
  });

  const deletePostMutation = api.useMutation(["deletePost"], {
    onSuccess: () => {
      console.log("Post deleted!");

      // TODO: add official invalidation
      // api.query.invalidate(["posts"]);

      // NOTE: this is a workaround for invalidating the query
      invalidateQuery("posts");
    },
    onError: (e) => {
      console.error(e);
    },
  });

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    createPostMutation.mutate({
      title,
      content,
    });
  };

  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-y-4">
      <h1 className="text-5xl font-extrabold tracking-tighter">Tauri + rspc + Prisma</h1>
      <p>Version: {version ?? "undefined"}</p>
      <form className="flex w-full max-w-sm flex-col gap-y-4" onSubmit={handleSubmit}>
        <input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="my title"
          className="rounded px-2 py-1 border"
          required
        />
        <input
          type="text"
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="my content"
          className="rounded px-2 py-1 border"
          required
        />
        <button
          type="submit"
          className="rounded px-2 py-1 bg-blue-500 text-white"
          disabled={createPostMutation.isPending}
        >
          {createPostMutation.isPending ? "Creating..." : "Create"}
        </button>
      </form>
      <div className="flex w-full max-w-sm flex-col gap-y-2">
        {posts?.map((post) => (
          <div
            key={post.id}
            className="px-2 py-1 flex justify-between items-center rounded border-2 border-white hover:border-blue-500"
          >
            <div>
              <p className="font-semibold">{post.title}</p>
              <p>{post.content}</p>
            </div>
            <div>
              <button
                className="rounded px-2 py-1 bg-red-500 text-white"
                onClick={() =>
                  deletePostMutation.mutate({
                    id: post.id,
                  })
                }
                disabled={deletePostMutation.isPending}
              >
                {deletePostMutation.isPending ? "Deleting..." : "Delete"}
              </button>
            </div>
          </div>
        ))}
        {posts?.length === 0 && <p>No posts found!</p>}
      </div>
    </div>
  );
}
