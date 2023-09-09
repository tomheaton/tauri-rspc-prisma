/* eslint-disable */
// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
  queries:
    | { key: "posts"; input: never; result: Post[] }
    | { key: "version"; input: never; result: string };
  mutations: { key: "createPost"; input: CreatePostInput; result: Post };
  subscriptions: never;
};

export type CreatePostInput = { title: string; content: string };

export type Post = { id: number; title: string; content: string };
