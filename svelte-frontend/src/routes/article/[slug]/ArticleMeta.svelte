<script>
	import { enhance } from '$app/forms';

	export let article;
	export let user;

	let profile_image = "https://api.realworld.io/images/smiley-cyrus.jpeg";
	if (article.author.image != null) {
		profile_image = article.author.image; 
	}
</script>

<div class="article-meta">
	<a href="/profile/@{article.author.username}">
		<img src={profile_image} alt={article.author.username} />
	</a>

	<div class="info">
		<a href="/profile/@{article.author.username}" class="author">{article.author.username}</a>
		<span class="date">
			{new Date(article.createdAt).toDateString()}
		</span>
	</div>

	{#if article.author.username === user?.username}
		<span>
			<a href="/editor/{article.slug}" class="btn btn-outline-secondary btn-sm">
				<i class="ion-edit" /> Edit Article
			</a>

			<form style="display: inline" use:enhance method="POST" action="?/deleteArticle">
				<button class="btn btn-outline-danger btn-sm">
					<i class="ion-trash-a" /> Delete Article
				</button>
			</form>
		</span>
	{/if}
</div>
