import request from '../utils/request';

export namespace ArticleApi {
  export interface ListRequest {
    page: number;
    per_page: number;
  }

  export interface ListResponseItem {
      id: number;
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      created_at: string;
      updated_at: string;
  }

  export interface ListResponse {
    page: number;
    per_page: number;
    total_pages: number;
    data: ListResponseItem[];
  }

  export interface DetailResponse {
      id: number;
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
      created_at: string;
      updated_at: string;
  }

  export interface CreateRequest {
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
  }

  export interface CreateResponse {
    id: number;
  }

  export interface UpdateRequest {
      category_id: number;
      title: string;
      author: string;
      source: string;
      source_url: string;
      thumbnail: string;
      summary: string;
      content: string;
  }

  export interface UpdateResponse {
    id: number;
  }

  export interface RemoveRequest {
      id: number;
  }

  export interface RemoveResponse {
  }
}

const articleApi = {
  list: (data: ArticleApi.ListRequest) => {
    return request.get<ArticleApi.ListResponse>('/api/v1/articles', { params: data });
  },
  detail: (id: number) => {
    return request.get<ArticleApi.DetailResponse>(`/api/v1/articles/${id}`);
  },
  remove: (id: number) => {
    return request.delete<ArticleApi.RemoveResponse>(`/api/v1/articles/${id}`);
  },
  create: (data: ArticleApi.CreateRequest) => {
    return request.post<ArticleApi.CreateResponse>('/api/v1/articles', data);
  },
  update: (id: number, data: ArticleApi.UpdateRequest) => {
    return request.put<ArticleApi.UpdateResponse>(`/api/v1/articles/${id}`, data);
  },
};

export default articleApi;

