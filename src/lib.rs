// This file is @generated by prost-build.
pub mod bilibili {
    pub mod account {
        pub mod fission {
            pub mod v1 {
                include!("bilibili.account.fission.v1.rs");
            }
        }
        pub mod interfaces {
            pub mod v1 {
                include!("bilibili.account.interfaces.v1.rs");
            }
        }
        pub mod service {
            pub mod v1 {
                include!("bilibili.account.service.v1.rs");
            }
        }
    }
    pub mod ad {
        pub mod v1 {
            include!("bilibili.ad.v1.rs");
        }
    }
    pub mod api {
        pub mod player {
            pub mod v1 {
                include!("bilibili.api.player.v1.rs");
            }
        }
        pub mod probe {
            pub mod v1 {
                include!("bilibili.api.probe.v1.rs");
            }
        }
        pub mod ticket {
            pub mod v1 {
                include!("bilibili.api.ticket.v1.rs");
            }
        }
    }
    pub mod app {
        pub mod archive {
            pub mod middleware {
                pub mod v1 {
                    include!("bilibili.app.archive.middleware.v1.rs");
                }
            }
            pub mod v1 {
                include!("bilibili.app.archive.v1.rs");
            }
        }
        pub mod card {
            pub mod v1 {
                include!("bilibili.app.card.v1.rs");
            }
        }
        pub mod click {
            pub mod v1 {
                include!("bilibili.app.click.v1.rs");
            }
        }
        pub mod distribution {
            include!("bilibili.app.distribution.rs");
            pub mod setting {
                pub mod download {
                    include!("bilibili.app.distribution.setting.download.rs");
                }
                pub mod dynamic {
                    include!("bilibili.app.distribution.setting.dynamic.rs");
                }
                pub mod experimental {
                    include!("bilibili.app.distribution.setting.experimental.rs");
                }
                pub mod home {
                    include!("bilibili.app.distribution.setting.home.rs");
                }
                pub mod internaldevice {
                    include!("bilibili.app.distribution.setting.internaldevice.rs");
                }
                pub mod night {
                    include!("bilibili.app.distribution.setting.night.rs");
                }
                pub mod other {
                    include!("bilibili.app.distribution.setting.other.rs");
                }
                pub mod pegasus {
                    include!("bilibili.app.distribution.setting.pegasus.rs");
                }
                pub mod play {
                    include!("bilibili.app.distribution.setting.play.rs");
                }
                pub mod privacy {
                    include!("bilibili.app.distribution.setting.privacy.rs");
                }
                pub mod search {
                    include!("bilibili.app.distribution.setting.search.rs");
                }
                pub mod story {
                    include!("bilibili.app.distribution.setting.story.rs");
                }
            }
        }
        pub mod dynamic {
            pub mod common {
                include!("bilibili.app.dynamic.common.rs");
            }
            pub mod v1 {
                include!("bilibili.app.dynamic.v1.rs");
            }
            pub mod v2 {
                include!("bilibili.app.dynamic.v2.rs");
            }
        }
        pub mod home {
            pub mod v1 {
                include!("bilibili.app.home.v1.rs");
            }
        }
        pub mod interfaces {
            pub mod v1 {
                include!("bilibili.app.interfaces.v1.rs");
            }
        }
        pub mod listener {
            pub mod v1 {
                include!("bilibili.app.listener.v1.rs");
            }
        }
        pub mod mine {
            pub mod v1 {
                include!("bilibili.app.mine.v1.rs");
            }
        }
        pub mod playeronline {
            pub mod v1 {
                include!("bilibili.app.playeronline.v1.rs");
            }
        }
        pub mod playerunite {
            pub mod pgcanymodel {
                include!("bilibili.app.playerunite.pgcanymodel.rs");
            }
            pub mod pugvanymodel {
                include!("bilibili.app.playerunite.pugvanymodel.rs");
            }
            pub mod ugcanymodel {
                include!("bilibili.app.playerunite.ugcanymodel.rs");
            }
            pub mod v1 {
                include!("bilibili.app.playerunite.v1.rs");
            }
        }
        pub mod playurl {
            pub mod v1 {
                include!("bilibili.app.playurl.v1.rs");
            }
        }
        pub mod resource {
            pub mod privacy {
                pub mod v1 {
                    include!("bilibili.app.resource.privacy.v1.rs");
                }
            }
            pub mod v1 {
                include!("bilibili.app.resource.v1.rs");
            }
        }
        pub mod search {
            pub mod v2 {
                include!("bilibili.app.search.v2.rs");
            }
        }
        pub mod show {
            pub mod gateway {
                pub mod v1 {
                    include!("bilibili.app.show.gateway.v1.rs");
                }
            }
            pub mod mixture {
                pub mod v1 {
                    include!("bilibili.app.show.mixture.v1.rs");
                }
            }
            pub mod popular {
                pub mod v1 {
                    include!("bilibili.app.show.popular.v1.rs");
                }
            }
            pub mod rank {
                pub mod v1 {
                    include!("bilibili.app.show.rank.v1.rs");
                }
            }
            pub mod region {
                pub mod v1 {
                    include!("bilibili.app.show.region.v1.rs");
                }
            }
        }
        pub mod space {
            pub mod v1 {
                include!("bilibili.app.space.v1.rs");
            }
        }
        pub mod splash {
            pub mod v1 {
                include!("bilibili.app.splash.v1.rs");
            }
        }
        pub mod topic {
            pub mod v1 {
                include!("bilibili.app.topic.v1.rs");
            }
        }
        pub mod view {
            pub mod v1 {
                include!("bilibili.app.view.v1.rs");
            }
        }
        pub mod viewunite {
            pub mod common {
                include!("bilibili.app.viewunite.common.rs");
            }
            pub mod pgcanymodel {
                include!("bilibili.app.viewunite.pgcanymodel.rs");
            }
            pub mod pugvanymodel {
                include!("bilibili.app.viewunite.pugvanymodel.rs");
            }
            pub mod ugcanymodel {
                include!("bilibili.app.viewunite.ugcanymodel.rs");
            }
            pub mod v1 {
                include!("bilibili.app.viewunite.v1.rs");
            }
        }
        pub mod wall {
            pub mod v1 {
                include!("bilibili.app.wall.v1.rs");
            }
        }
    }
    pub mod broadcast {
        pub mod live {
            include!("bilibili.broadcast.live.rs");
            pub mod multi_conn {
                include!("bilibili.broadcast.live.multi_conn.rs");
            }
            pub mod pk {
                include!("bilibili.broadcast.live.pk.rs");
            }
            pub mod pmms {
                include!("bilibili.broadcast.live.pmms.rs");
            }
            pub mod universal_interact {
                include!("bilibili.broadcast.live.universal_interact.rs");
            }
            pub mod voice_room {
                include!("bilibili.broadcast.live.voice_room.rs");
            }
        }
        pub mod message {
            pub mod archive {
                include!("bilibili.broadcast.message.archive.rs");
            }
            pub mod bgroup {
                include!("bilibili.broadcast.message.bgroup.rs");
            }
            pub mod gamecenter {
                include!("bilibili.broadcast.message.gamecenter.rs");
            }
            pub mod im {
                include!("bilibili.broadcast.message.im.rs");
            }
            pub mod main {
                include!("bilibili.broadcast.message.main.rs");
            }
            pub mod mall {
                include!("bilibili.broadcast.message.mall.rs");
            }
            pub mod push {
                include!("bilibili.broadcast.message.push.rs");
            }
            pub mod reply {
                include!("bilibili.broadcast.message.reply.rs");
            }
            pub mod topic {
                include!("bilibili.broadcast.message.topic.rs");
            }
            pub mod tv {
                include!("bilibili.broadcast.message.tv.rs");
            }
        }
        pub mod v1 {
            include!("bilibili.broadcast.v1.rs");
        }
        pub mod v2 {
            include!("bilibili.broadcast.v2.rs");
        }
    }
    pub mod cheese {
        pub mod gateway {
            pub mod player {
                pub mod v1 {
                    include!("bilibili.cheese.gateway.player.v1.rs");
                }
            }
        }
    }
    pub mod community {
        pub mod interfacess {
            pub mod biligram {
                pub mod v1 {
                    include!("bilibili.community.interfacess.biligram.v1.rs");
                }
            }
            pub mod cosmoconn {
                pub mod v1 {
                    include!("bilibili.community.interfacess.cosmoconn.v1.rs");
                }
            }
        }
        pub mod service {
            pub mod cert {
                pub mod v1 {
                    include!("bilibili.community.service.cert.v1.rs");
                }
            }
            pub mod dm {
                pub mod v1 {
                    include!("bilibili.community.service.dm.v1.rs");
                }
            }
            pub mod govern {
                pub mod v1 {
                    include!("bilibili.community.service.govern.v1.rs");
                }
            }
        }
    }
    pub mod creative_tool {
        pub mod editor {
            pub mod v2 {
                include!("bilibili.creative_tool.editor.v2.rs");
            }
        }
    }
    pub mod dagw {
        pub mod component {
            pub mod avatar {
                pub mod common {
                    include!("bilibili.dagw.component.avatar.common.rs");
                }
                pub mod v1 {
                    include!("bilibili.dagw.component.avatar.v1.rs");
                    pub mod plugin {
                        include!("bilibili.dagw.component.avatar.v1.plugin.rs");
                    }
                }
            }
        }
    }
    pub mod dynamic {
        pub mod common {
            include!("bilibili.dynamic.common.rs");
        }
        pub mod interfaces {
            pub mod campus {
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.campus.v1.rs");
                }
            }
            pub mod feed {
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.feed.v1.rs");
                }
            }
            pub mod vote {
                pub mod v1 {
                    include!("bilibili.dynamic.interfaces.vote.v1.rs");
                }
            }
        }
    }
    pub mod gaia {
        pub mod gw {
            include!("bilibili.gaia.gw.rs");
        }
    }
    pub mod im {
        pub mod customer {
            pub mod independent {
                include!("bilibili.im.customer.independent.rs");
            }
            pub mod interfaces {
                include!("bilibili.im.customer.interfaces.rs");
            }
            pub mod model {
                include!("bilibili.im.customer.model.rs");
            }
        }
        pub mod gateway {
            pub mod interfaces {
                pub mod v1 {
                    include!("bilibili.im.gateway.interfaces.v1.rs");
                }
            }
        }
        pub mod interfaces {
            pub mod v1 {
                include!("bilibili.im.interfaces.v1.rs");
            }
        }
        pub mod r#type {
            include!("bilibili.im.r#type.rs");
        }
    }
    pub mod live {
        pub mod app {
            pub mod interfaces {
                pub mod api {
                    pub mod grpc {
                        pub mod v1 {
                            include!("bilibili.live.app.interfaces.api.grpc.v1.rs");
                        }
                    }
                }
            }
            pub mod room {
                pub mod v1 {
                    include!("bilibili.live.app.room.v1.rs");
                }
            }
        }
        pub mod approom {
            pub mod api {
                pub mod grpc {
                    pub mod v1 {
                        include!("bilibili.live.approom.api.grpc.v1.rs");
                    }
                }
            }
        }
        pub mod rtc {
            include!("bilibili.live.rtc.rs");
            pub mod datachannel {
                include!("bilibili.live.rtc.datachannel.rs");
                pub mod report {
                    include!("bilibili.live.rtc.datachannel.report.rs");
                }
            }
        }
        pub mod xroom_extend {
            pub mod api {
                pub mod v1 {
                    include!("bilibili.live.xroom_extend.api.v1.rs");
                }
            }
        }
    }
    pub mod main {
        pub mod community {
            pub mod reply {
                pub mod v1 {
                    include!("bilibili.main.community.reply.v1.rs");
                }
                pub mod v2 {
                    include!("bilibili.main.community.reply.v2.rs");
                }
            }
        }
    }
    pub mod mall {
        pub mod tab3 {
            pub mod dynamic {
                pub mod v1 {
                    include!("bilibili.mall.tab3.dynamic.v1.rs");
                }
            }
            pub mod playerunite {
                pub mod ugcanymodel {
                    include!("bilibili.mall.tab3.playerunite.ugcanymodel.rs");
                }
                pub mod v1 {
                    include!("bilibili.mall.tab3.playerunite.v1.rs");
                }
            }
            pub mod viewunite {
                pub mod common {
                    include!("bilibili.mall.tab3.viewunite.common.rs");
                }
                pub mod ugcanymodel {
                    include!("bilibili.mall.tab3.viewunite.ugcanymodel.rs");
                }
                pub mod v1 {
                    include!("bilibili.mall.tab3.viewunite.v1.rs");
                }
            }
        }
    }
    pub mod metadata {
        include!("bilibili.metadata.rs");
        pub mod device {
            include!("bilibili.metadata.device.rs");
        }
        pub mod fawkes {
            include!("bilibili.metadata.fawkes.rs");
        }
        pub mod locale {
            include!("bilibili.metadata.locale.rs");
        }
        pub mod network {
            include!("bilibili.metadata.network.rs");
        }
        pub mod parabox {
            include!("bilibili.metadata.parabox.rs");
        }
        pub mod restriction {
            include!("bilibili.metadata.restriction.rs");
        }
    }
    pub mod pagination {
        include!("bilibili.pagination.rs");
    }
    pub mod pangu {
        pub mod gallery {
            pub mod v1 {
                include!("bilibili.pangu.gallery.v1.rs");
            }
        }
    }
    pub mod pgc {
        pub mod gateway {
            pub mod player {
                pub mod v1 {
                    include!("bilibili.pgc.gateway.player.v1.rs");
                }
                pub mod v2 {
                    include!("bilibili.pgc.gateway.player.v2.rs");
                }
            }
        }
    }
    pub mod playershared {
        include!("bilibili.playershared.rs");
    }
    pub mod pmms {
        pub mod v1 {
            include!("bilibili.pmms.v1.rs");
        }
    }
    pub mod polymer {
        pub mod app {
            pub mod search {
                pub mod v1 {
                    include!("bilibili.polymer.app.search.v1.rs");
                }
            }
        }
        pub mod chronos {
            pub mod v1 {
                include!("bilibili.polymer.chronos.v1.rs");
            }
        }
        pub mod community {
            pub mod govern {
                pub mod v1 {
                    include!("bilibili.polymer.community.govern.v1.rs");
                }
            }
        }
        pub mod contract {
            include!("bilibili.polymer.contract.rs");
        }
        pub mod list {
            include!("bilibili.polymer.list.rs");
        }
    }
    pub mod relation {
        pub mod interfaces {
            include!("bilibili.relation.interfaces.rs");
        }
    }
    pub mod rpc {
        include!("bilibili.rpc.rs");
    }
    pub mod vas {
        pub mod garb {
            pub mod model {
                include!("bilibili.vas.garb.model.rs");
            }
            pub mod service {
                include!("bilibili.vas.garb.service.rs");
            }
        }
    }
    pub mod vega {
        pub mod deneb {
            pub mod v1 {
                include!("bilibili.vega.deneb.v1.rs");
            }
        }
    }
}
pub mod pgc {
    pub mod biz {
        include!("pgc.biz.rs");
    }
    pub mod gateway {
        pub mod vega {
            pub mod v1 {
                include!("pgc.gateway.vega.v1.rs");
            }
        }
    }
}
